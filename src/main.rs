use clap::Parser;
use std::fs;
use std::process::Command;

const FASTAPI_CODE: &str = r#"from fastapi import FastAPI

app = FastAPI()

@app.get("/")
async def default():
    return {"response":"on"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
"#;

#[derive(Parser, Debug)]
#[command(name = "quickstart")]
#[command(about = "Quickly scaffold projects")]
struct QuickCommand {
    project_name: String,

    project_type: String,

    #[arg(long)]
    fastapi: bool,

    #[arg(long, default_value = "python")]
    backend_framework: String,

    #[arg(long, default_value = "react")]
    frontend_framework: String,
}

fn main() {
    let args = QuickCommand::parse();

    match args.project_type.as_str() {
        "backend" => create_backend(&args.project_name, &args.backend_framework, &args.fastapi),
        "frontend" => create_frontend(&args.project_name, &args.frontend_framework),
        _ => eprintln!("Unknown project type: {}", args.project_type),
    }
}

fn create_backend(project_name: &str, framework: &str, fastapi: &bool) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Project backend: {project_name}");

    let project_path = current_dir.join(project_name);
    fs::create_dir_all(&project_path).expect("Failed to create directory");
    println!("Created backend folder at: {}", project_path.display());

    match framework.to_lowercase().as_str() {
        "python" => {
            let python_project_path = project_path.join("backend");
            fs::create_dir_all(&python_project_path).expect("Failed to create directory");

            // cd into the created directory and create venv

            Command::new("python")
                .arg("-m")
                .arg("venv")
                .arg("venv")
                .current_dir(&python_project_path)
                .status()
                .expect("Failed to create venv");
            println!("venv initialized!");

            let main_py_path = python_project_path.join("main.py");
            fs::write(&main_py_path, "").expect("Failed to create main.py");
            println!("main.py created.");

            if *fastapi {
                println!("FastAPI boilerplate generated.");
                let server_py_path = python_project_path.join("server.py");
                let requirements_path = python_project_path.join("requirements.txt");

                fs::write(&server_py_path, FASTAPI_CODE).expect("Failed to create server.py");
                fs::write(&requirements_path, "fastapi")
                    .expect("Failed to create requirements.txt as a dependency");

                println!("server.py and requirements.txt created.");

                let pip_path = python_project_path
                    .join("venv")
                    .join("Scripts")
                    .join("pip.exe");

                Command::new(&pip_path)
                    .arg("install")
                    .arg("-r")
                    .arg("requirements.txt")
                    .current_dir(&python_project_path)
                    .status()
                    .expect("Failed to install dependencies.");

                println!("dependencies installed!");
            }
        }
        "node" => {
            println!("Creating Node project...");

            let npm_path = "C:\\Program Files\\nodejs\\npm.cmd";

            Command::new(npm_path)
                .arg("init")
                .arg("-y")
                .current_dir(&project_path)
                .status()
                .expect("Failed to create a node project.");

            println!(
                "Node.js project created successfully at {}/{}",
                current_dir.display(),
                project_name
            );

            Command::new(npm_path)
                .arg("install")
                .arg("express")
                .arg("jsonwebtoken")
                .current_dir(&project_path)
                .status()
                .expect("Failed to install dependencies");

            println!("Node.js dependencies installed.");
        }
        _ => {
            println!("No framework provided")
        }
    }
}

fn create_frontend(project_name: &str, framework: &str) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    match framework.to_lowercase().as_str() {
        "react" => {
            println!("Creating React (Vite) project...");

            let npx_path = "C:\\Program Files\\nodejs\\npx.cmd";

            Command::new(npx_path)
                .arg("create-vite@latest")
                .arg(project_name)
                .current_dir(&current_dir)
                .status()
                .expect("Failed to create a vite project.");

            println!(
                "React project created successfully at {}/{}",
                current_dir.display(),
                project_name
            );

            Command::new("npm")
                .arg("install")
                .current_dir(current_dir.join(project_name))
                .status()
                .expect("Failed to install dependencies");

            println!("React (Vite) dependencies installed.");
        }
        "next" => {
            println!("Creating Next.js project...");

            Command::new("npx")
                .arg("create-next-app@latest")
                .arg(project_name)
                .arg("--typescript")
                .arg("--eslint")
                .current_dir(&current_dir)
                .status()
                .expect("Failed to create a next project.");

            println!(
                "Next.js project created at {}/{}",
                current_dir.display(),
                project_name
            );

            Command::new("npm")
                .arg("install")
                .current_dir(current_dir.join(project_name))
                .status()
                .expect("Failed to install Next.js dependencies");

            println!("Next.js dependencies installed.");
        }
        _ => println!("No framework provided"),
    }
}
