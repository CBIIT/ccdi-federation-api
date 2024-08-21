# Invalid Routes

All responses that do not match an endpoint below should return a Not Found
(`404`) response. The body of this response should be the `responses.Errors`
JSON object with one `responses.error.Kind` where the `Kind` matches the
`InvalidRoute` error.
