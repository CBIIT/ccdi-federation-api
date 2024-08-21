import * as d3 from "d3";
import _ from "lodash";

interface BarPlotEntry {
  group: string;
  count: number;
  value: string;
}

const colorMap: { [server: string]: string } = {
  // St. Jude red, provided by design.stjude.cloud.
  "St. Jude Children's Research Hospital": "#C10F3A",

  // Children's Hospital of Philadelphia light blue, provided by William Khan.
  "KidsFirst DRC": "#7DC4E2",

  // UCSC gold, provided by Ellen Kephart.
  "Treehouse Childhood Cancer Initiative": "#FDC700",

  // Pediatric Cancer Data Commons medium blue, provided by Brian Furner.
  "Pediatric Cancer Data Commons": "#155F83",
};

interface Config {
  height: number;
  width: number;
  margin: {
    top: number;
    right: number;
    bottom: number;
    left: number;
  };
  scale: number;
}

const defaultConfig: Config = {
  height: 500,
  width: 700,
  margin: {
    top: 30,
    right: 30,
    bottom: 90,
    left: 50,
  },
  scale: 1.1,
};

export function bar(
  id: string,
  title: string,
  data: [BarPlotEntry],
  options: any = {}
) {
  let opt: Config = _.merge({}, defaultConfig, options);

  const innerWidth = opt.width - opt.margin.left - opt.margin.right,
    innerHeight = opt.height - opt.margin.top - opt.margin.bottom;

  let counts: { [key: string]: number } = {};

  for (const entry of data) {
    counts[entry.value] = (counts[entry.value] || 0) + entry.count;
  }

  let maxCount = d3.max(Object.values(counts)) || 0;
  let values = Array.from(Object.entries(counts))
    .sort((a, b) => b[1] - a[1])
    .map(([k, _]) => k);

  let groups = d3.union(data.map((d) => d.group));

  // Determine the series that need to be stacked.
  const seriesFn = d3
    .stack()
    .keys(groups)
    // @ts-ignore
    .value(([, D], key) => D.get(key)?.count || 0);

  const series = seriesFn(
    // @ts-ignore
    //
    // NOTE: this coerces just fine.
    d3.index(
      data,
      (entry) => entry.value,
      (entry) => entry.group
    )
  );

  d3.select(id).selectAll("*").remove();

  const svg = d3
    .select(id)
    .append("svg")
    .attr("width", innerWidth + opt.margin.left + opt.margin.right)
    .attr("height", innerHeight + opt.margin.top + opt.margin.bottom)
    .attr("style", "max-width: 100%; height: auto;")
    .append("g")
    .attr("transform", `translate(${opt.margin.left},${opt.margin.top})`);

  const x = d3.scaleBand().domain(values).range([0, innerWidth]).padding(0.2);

  svg
    .append("g")
    .attr("transform", `translate(0, ${innerHeight})`)
    .call(d3.axisBottom(x).tickSizeOuter(0))
    .selectAll("text")
    .attr("transform", "translate(-10,0)rotate(-45)")
    .style("text-anchor", "end");

  const y = d3
    .scaleLinear()
    .domain([0, maxCount * opt.scale])
    .range([innerHeight, 0]);

  svg.append("g").call(d3.axisLeft(y));

  svg
    .append("g")
    .selectAll()
    .data(series)
    .join("g")
    .attr("fill", (series) => {
      let color = colorMap[series.key];

      if (color === undefined || color === null) {
        console.warn(`"${series.key}" needs a color to be defined!`);
      }

      return color || "green";
    })
    .selectAll("rect")
    .data((series) => series)
    .join("rect")
    // @ts-ignore
    .attr("x", (d) => x(d.data[0]))
    .attr("y", (d) => y(d[1]))
    .attr("height", (d) => y(d[0]) - y(d[1]))
    .attr("width", x.bandwidth());

  const size = 10;
  svg
    .selectAll("squares")
    .data(groups)
    .enter()
    .append("rect")
    .attr("x", innerWidth - 275)
    .attr("y", function (d, i) {
      return 20 + i * (size + 5);
    })
    .attr("width", size)
    .attr("height", size)
    .style("fill", function (d) {
      return colorMap[d];
    });

  svg
    .selectAll("labels")
    .data(groups)
    .enter()
    .append("text")
    .attr("x", innerWidth - 250)
    .attr("y", function (d, i) {
      return 25 + i * (size + 5) + size / 2;
    })
    .style("fill", function (d) {
      return colorMap[d];
    })
    .text(function (d) {
      return d;
    })
    .attr("text-anchor", "left")
    .attr("font-size", "14")
    .attr("alignment-baseline", "middle");

  svg
    .append("text")
    .attr("x", innerWidth / 2)
    .attr("y", 0)
    .attr("text-anchor", "middle")
    .attr("class", "dark:fill-slate-100")
    .attr("font-weight", "bold")
    .text(title);
}
