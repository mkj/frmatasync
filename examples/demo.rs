use frmatasync::Frmatasync;
use embedded_io_adapters::tokio_1::FromTokio;

#[tokio::main]
async fn main() {

    let stdout = FromTokio::new(tokio::io::stdout());

    let mut f = Frmatasync::new(stdout);

    let zzz = [1,2,3];

    writeln!(f, "helaaaaaaaaaaaaaalo {zzz:?}").await.unwrap();
    writeln!(f, "more").await.unwrap();

}
