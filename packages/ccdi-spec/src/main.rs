use std::fs::File;
use std::io;
use std::net::Ipv4Addr;
use std::path::PathBuf;

use actix_web::error::QueryPayloadError;
use actix_web::middleware::Logger;
use actix_web::rt;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::QueryConfig;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use clap::Parser;
use clap::Subcommand;
use itertools::Itertools as _;
use log::info;
use log::LevelFilter;
use server::routes::file;
use server::routes::organization;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use ccdi_models as models;
use ccdi_openapi as api;
use ccdi_server as server;

use api::Api;

use server::responses::error;
use server::responses::Errors;
use server::routes::info;
use server::routes::metadata;
use server::routes::namespace;
use server::routes::sample;
use server::routes::subject;

mod utils;

use utils::markdown;

const ERROR_EXIT_CODE: i32 = 1;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Entity {
    /// A subject.
    Subject,

    /// A sample.
    Sample,

    /// A file.
    File,
}

/// An error related to the main program.
#[derive(Debug)]
pub enum Error {
    /// A file already exists at the specified location.
    FileExists(PathBuf),

    /// An input/output error.
    IoError(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileExists(path) => write!(f, "file already exists: {}", path.display()),
            Error::IoError(err) => write!(f, "i/o error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Parser)]
pub struct GenerateArgs {
    /// A path to write the output to.
    #[arg(short = 'o')]
    output: Option<PathBuf>,

    /// Whether to force the output file to be overwritten (if it exists).
    #[arg(short, long)]
    force: bool,
}

#[derive(Clone, Debug, clap::ValueEnum)]
#[clap(rename_all = "PascalCase")]
pub enum ResponseType {
    Samples,
    Sample,
    SamplesByCount,
    Subjects,
    Subject,
    SubjectsByCount,
    Files,
    Namespaces,
    Namespace,
    Organizations,
    Organization,
    Summary,
    Information,
    FieldDescriptions,
    Errors,
}

fn parse_response(
    text: &str,
    response_type: ResponseType,
) -> Result<(), Box<dyn std::error::Error>> {
    match response_type {
        ResponseType::Samples => {
            serde_json::from_str::<server::responses::Samples>(text).map(|_| ())?;
        }
        ResponseType::Sample => {
            serde_json::from_str::<server::responses::Sample>(text).map(|_| ())?;
        }
        ResponseType::SamplesByCount => {
            serde_json::from_str::<server::responses::by::count::sample::Response>(text)
                .map(|_| ())?;
        }
        ResponseType::Subjects => {
            serde_json::from_str::<server::responses::Subjects>(text).map(|_| ())?;
        }
        ResponseType::Subject => {
            serde_json::from_str::<server::responses::Subject>(text).map(|_| ())?;
        }
        ResponseType::SubjectsByCount => {
            serde_json::from_str::<server::responses::by::count::subject::Results>(text)
                .map(|_| ())?;
        }
        ResponseType::Files => {
            serde_json::from_str::<server::responses::Files>(text).map(|_| ())?;
        }
        ResponseType::Namespaces => {
            serde_json::from_str::<server::responses::Namespaces>(text).map(|_| ())?;
        }
        ResponseType::Namespace => {
            serde_json::from_str::<server::responses::Namespace>(text).map(|_| ())?;
        }
        ResponseType::Organizations => {
            serde_json::from_str::<server::responses::Organizations>(text).map(|_| ())?;
        }
        ResponseType::Organization => {
            serde_json::from_str::<server::responses::Organization>(text).map(|_| ())?;
        }
        ResponseType::Summary => {
            serde_json::from_str::<server::responses::Summary>(text).map(|_| ())?;
        }
        ResponseType::Information => {
            serde_json::from_str::<server::responses::Information>(text).map(|_| ())?;
        }
        ResponseType::FieldDescriptions => {
            serde_json::from_str::<server::responses::metadata::FieldDescriptions>(text)
                .map(|_| ())?;
        }
        ResponseType::Errors => {
            serde_json::from_str::<server::responses::Errors>(text).map(|_| ())?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
pub struct CheckArgs {
    /// The URL to retreive.
    url: String,

    /// The type of response to parse.
    response_type: ResponseType,
}

#[derive(Debug, Parser)]
pub struct ServeArgs {
    /// Number of subjects for the server to generate.
    #[arg(default_value_t = 100)]
    number_of_subjects: usize,

    /// Number of samples for the server to generate.
    #[arg(default_value_t = 100)]
    number_of_samples: usize,

    /// Number of files for the server to generate.
    #[arg(default_value_t = 1000)]
    number_of_files: usize,

    /// A path to write the output to.
    #[arg(short = 'p', default_value_t = 8000)]
    port: u16,
}

#[derive(Debug, Parser)]
pub struct WikiArgs {
    /// The API entity for which to generate a wiki page.
    entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks that a URL matches the specification.
    Check(CheckArgs),

    /// Generate the OpenAPI specification.
    Generate(GenerateArgs),

    /// Runs the test server.
    Serve(ServeArgs),

    /// Generates the documentation for the wiki page.
    Wiki(WikiArgs),
}

/// A program to generate the Childhood Cancer Data Initiative OpenAPI
/// specification.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: Command,
}

fn get_output(path: Option<PathBuf>, force: bool) -> Result<Box<dyn std::io::Write>, Error> {
    match path {
        Some(path) => {
            if !force && path.exists() {
                return Err(Error::FileExists(path));
            }

            File::create(path)
                .map(|file| Box::new(file) as Box<dyn std::io::Write>)
                .map_err(Error::IoError)
        }
        None => Ok(Box::new(std::io::stdout())),
    }
}

fn inner() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    match args.command {
        Command::Check(args) => {
            let response = reqwest::blocking::get(&args.url)?;
            let text = response.text()?;
            parse_response(&text, args.response_type)?;
            println!("Success!");
        }
        Command::Generate(args) => {
            let api = Api::openapi();
            let mut writer = get_output(args.output, args.force)?;
            write!(writer, "{}", api.to_yaml()?)?;
        }
        Command::Serve(args) => {
            info!("Starting server at http://localhost:{}", args.port);

            rt::System::new().block_on(
                HttpServer::new(move || {
                    let subjects = subject::Store::random(args.number_of_subjects);

                    let samples = sample::Store::random(
                        args.number_of_samples,
                        subjects.subjects.lock().unwrap(),
                    );

                    let files =
                        file::Store::random(args.number_of_files, samples.samples.lock().unwrap());

                    let subjects = Data::new(subjects);
                    let samples = Data::new(samples);
                    let files = Data::new(files);

                    App::new()
                        .app_data(QueryConfig::default().error_handler(|err, _| {
                            match err {
                                QueryPayloadError::Deserialize(err) => {
                                    Errors::new(vec![error::Kind::invalid_parameters(
                                        None,
                                        err.to_string(),
                                    )])
                                    .into()
                                }
                                _ => todo!(),
                            }
                        }))
                        .wrap(Logger::default())
                        .configure(subject::configure(subjects))
                        .configure(sample::configure(samples))
                        .configure(file::configure(files))
                        .configure(metadata::configure())
                        .configure(namespace::configure())
                        .configure(organization::configure())
                        .configure(info::configure())
                        .service(
                            SwaggerUi::new("/swagger-ui/{_:.*}")
                                .url("/api-docs/openapi.json", Api::openapi()),
                        )
                        .default_service(web::to(|req: HttpRequest| async move {
                            HttpResponse::NotFound().json(Errors::from(error::Kind::invalid_route(
                                req.method().to_string(),
                                req.path().to_string(),
                            )))
                        }))
                })
                .bind((Ipv4Addr::UNSPECIFIED, args.port))?
                .run(),
            )?;
        }
        Command::Wiki(args) => {
            let fields = match args.entity {
                Entity::Subject => {
                    models::metadata::field::description::harmonized::subject::get_field_descriptions()
                }
                Entity::Sample => {
                    models::metadata::field::description::harmonized::sample::get_field_descriptions(
                    )
                }
                Entity::File => {
                    models::metadata::field::description::harmonized::file::get_field_descriptions(
                    )
                }
            };

            print!(
                "{}",
                fields.into_iter().map(markdown::Section::from).join("\n")
            );
        }
    }

    Ok(())
}

fn main() {
    match inner() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
