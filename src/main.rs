use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use structopt::StructOpt;

use p3d::{AlgoType, P3DError};
                       
#[derive(StructOpt, Debug)]
#[structopt(name = "p3d-cli")]
struct Cli {
    /// 3D hash algorithm.
    /// Supported: grid2d, grid2d_v2, grid2d_v3, grid2d_v3a
    #[structopt(short, long)]
    algo: String,

    /// Number of cells in Grid2d algorithm
    #[structopt(short, long)]
    grid: i16,

    /// Number of sections in Grid2d algorithm
    #[structopt(short, long)]
    sect: i16,

    /// Number of hashes/points for relevant algorithms (e.g., grid2d_v3, grid2d_v3a)
    #[structopt(default_value = "10", short, long)]
    depth: usize,

    /// The path to the input file (.obj or .glb)
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    let args = Cli::from_args();

    let mut file = File::open(&args.infile)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_extension = args
        .infile
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let result_hashes: Result<Vec<String>, P3DError> = match file_extension.to_lowercase().as_str() {
        "obj" => {
            println!("Processing OBJ file: {:?}", &args.infile);
            match args.algo.as_str() {
                "grid2d" => p3d::p3d_process(buffer.as_slice(), AlgoType::Grid2d, args.grid, args.sect, None),
                "grid2d_v2" => p3d::p3d_process(buffer.as_slice(), AlgoType::Grid2dV2, args.grid, args.sect, None),
                "grid2d_v3" => p3d::p3d_process_n(buffer.as_slice(), AlgoType::Grid2dV3, args.depth, args.grid, args.sect, None),
                "grid2d_v3a" => p3d::p3d_process_n(buffer.as_slice(), AlgoType::Grid2dV3a, args.depth, args.grid, args.sect, None),
                _ => {
                    eprintln!("Unknown algorithm for OBJ: {}", args.algo);
                    return Err(Box::from("Unknown algorithm specified for OBJ file type."));
                }
            }
        }
        "glb" => {
            println!("Processing GLB file: {:?}", &args.infile);
            match args.algo.as_str() {
                "grid2d" => p3d::p3d_process_glb(buffer.as_slice(), AlgoType::Grid2d, args.depth, args.grid, args.sect, None),
                "grid2d_v2" => p3d::p3d_process_glb(buffer.as_slice(), AlgoType::Grid2dV2, args.depth, args.grid, args.sect, None),
                "grid2d_v3" => p3d::p3d_process_glb(buffer.as_slice(), AlgoType::Grid2dV3, args.depth, args.grid, args.sect, None),
                "grid2d_v3a" => p3d::p3d_process_glb(buffer.as_slice(), AlgoType::Grid2dV3a, args.depth, args.grid, args.sect, None),
                _ => {
                    eprintln!("Unknown algorithm for GLB: {}", args.algo);
                     return Err(Box::from("Unknown algorithm specified for GLB file type."));
                }
            }
        }
        _ => {
            eprintln!(
                "Unsupported file extension: {}. Please use .obj or .glb",
                file_extension
            );
            return Err(Box::from("Unsupported file type."));
        }
    };

    match result_hashes {
        Ok(hashes) => {
            for h in &hashes {
                println!("{:?}", h);
            }
        }
        Err(e) => {
            eprintln!("Error processing file: {:?}", e);
            return Err(Box::from(format!("Processing error: {:?}",e)));
        }
    }

    let duration = start_time.elapsed();
    println!("Processing time: {:.3?}", duration);

    Ok(())
}
