use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Main command
struct MyApp {
    #[argh(subcommand)]
    command: MainCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum MainCommands {
    Backend(BackendCmd),
    Frontend(FrontendCmd),
    Sql(SqlCmd),
    Project(ProjectCmd),
}

/// Backend command and its subcommands
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "b")]
struct BackendCmd {
    #[argh(subcommand)]
    subcommand: BackendCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum BackendCommands {
    Api(Api),
    Web(BWeb),
    Rest(Rest),
    Lib(Lib),
}

#[derive(FromArgs, Debug)]
/// Backend api
#[argh(subcommand, name = "api")]
struct Api {}

#[derive(FromArgs, Debug)]
/// Backend web
#[argh(subcommand, name = "web")]
struct BWeb {}

#[derive(FromArgs, Debug)]
/// Backend rest
#[argh(subcommand, name = "rest")]
struct Rest {}

#[derive(FromArgs, Debug)]
/// Backend lib
#[argh(subcommand, name = "lib")]
struct Lib {}

/// Frontend command and its subcommands
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "f")]
struct FrontendCmd {
    #[argh(subcommand)]
    subcommand: FrontendCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum FrontendCommands {
    Comp(Comp),
    Web(Web),
}

#[derive(FromArgs, Debug)]
/// Build the frontend
#[argh(subcommand, name = "comp")]
struct Comp {}

#[derive(FromArgs, Debug)]
/// Build the frontend
#[argh(subcommand, name = "web")]
struct Web {}

/// Frontend command and its subcommands
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "s")]
struct SqlCmd {
    #[argh(subcommand)]
    subcommand: SqlCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SqlCommands {
    Table(Table),
    Data(Data),
}

#[derive(FromArgs, Debug)]
/// Sql table
#[argh(subcommand, name = "table")]
struct Table {}

#[derive(FromArgs, Debug)]
/// Sql data
#[argh(subcommand, name = "data")]
struct Data {}

/// Project command and its subcommands
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "p")]
struct ProjectCmd {
    #[argh(subcommand)]
    subcommand: ProjectCommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum ProjectCommands {
    Req(Req),
    Tech(Tech),
}

#[derive(FromArgs, Debug)]
/// Create a new project
#[argh(subcommand, name = "req")]
struct Req {}

#[derive(FromArgs, Debug)]
/// Delete an existing project
#[argh(subcommand, name = "tech")]
struct Tech {}

#[tokio::main]
async fn main() -> Result<()> {
    let app: MyApp = argh::from_env();
    Ok(())
}
