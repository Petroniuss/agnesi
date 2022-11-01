use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use color_eyre::Result;
use color_eyre::eyre::{ContextCompat, eyre};
use ethers::abi::Param;
use ethers_solc::{Project, ProjectCompileOutput, ProjectPathsConfig};

#[allow(dead_code)]
pub(crate) fn compile_solidity_project() -> Result<ProjectCompileOutput> {
    let root = PathBuf::from("resources/solidity/");

    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()?;

    let project = Project::builder()
        .paths(paths)
        .set_auto_detect(true)
        .no_artifacts()
        .build()?;

    let output = project.compile()?;
    if !output.has_compiler_errors() {
        Ok(output.clone())
    } else {
        Err(eyre!(
            "Solidity compilation failed: {:#?}",
            output.output().errors
        ))
    }
}

#[allow(dead_code)]
pub(crate) fn display_contract_info(project: &ProjectCompileOutput) -> Result<()> {
    for contract_info in collect_contracts_info(project.clone())? {
        println!("{}", contract_info);
    }

    Ok(())
}

struct TypedParam(Param);
struct TypedParams(Vec<TypedParam>);
struct TypedOutput(Option<TypedParam>);
struct TypedFunctions(Vec<FunctionInfo>);
struct TypedConstructor(Option<FunctionInfo>);

struct ContractInfo {
    name: String,
    constructor: TypedConstructor,
    functions: TypedFunctions,
}

struct FunctionInfo {
    name: String,
    inputs: TypedParams,
    output: TypedOutput,
}

impl Display for ContractInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let inputs = self
            .functions
            .0
            .iter()
            .map(|x| format!("\t{}", x))
            .collect::<Vec<_>>()
            .join("\n");

        let output = format!("Contract: {}\n{}", &self.name, inputs);
        f.write_str(&output)
    }
}

impl Display for FunctionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = format!("{} :: {} -> {}", self.name, &self.inputs, self.output);
        f.write_str(&output)
    }
}

impl Display for TypedParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let params = &self.0;
        let inputs = params
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(", ");

        let output = format!("[{}]", inputs);
        f.write_str(&output)
    }
}

impl Display for TypedOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = if let Some(ref x) = self.0 {
            format!("{}", x)
        } else {
            "Void".to_string()
        };

        f.write_str(&output)
    }
}

impl Display for TypedParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let wrapped = &self.0;
        let name = &wrapped.name;
        let param_type = &wrapped.kind;

        let output = if !name.is_empty() {
            format!("{}: {}", name, param_type)
        } else {
            format!("{}", param_type)
        };

        f.write_str(&output)
    }
}

fn collect_contracts_info(project: ProjectCompileOutput) -> Result<Vec<ContractInfo>> {
    project
        .into_artifacts()
        .into_iter()
        .map(|(id, artifact)| {
            let name = id.name.clone();
            let abi = artifact.abi.context("No ABI found for artifact {name}")?;
            let contract = &abi.abi;

            let functions = contract.functions();
            let functions = functions.cloned();
            let constructor = contract.constructor();

            let constructor = constructor.map(|constructor| {
                let params = &constructor.inputs;

                let inputs = params
                    .into_iter()
                    .map(|x| TypedParam(x.clone()))
                    .collect::<Vec<_>>();
                let inputs = TypedParams(inputs);

                FunctionInfo {
                    name: "constructor".to_string(),
                    inputs,
                    output: TypedOutput(None),
                }
            });

            let functions = functions
                .into_iter()
                .map(|func| {
                    let name = &func.name;
                    let params = &func.inputs;

                    let inputs = params
                        .into_iter()
                        .map(|x| TypedParam(x.clone()))
                        .collect::<Vec<_>>();
                    let inputs = TypedParams(inputs);

                    let output = func.outputs.first().map(|x| TypedParam(x.clone()));
                    let output = TypedOutput(output);

                    Ok(FunctionInfo {
                        name: name.clone(),
                        inputs,
                        output,
                    })
                })
                .collect::<Result<Vec<_>>>();

            Ok(ContractInfo {
                name,
                constructor: TypedConstructor(constructor),
                functions: TypedFunctions(functions?),
            })
        })
        .collect::<Result<Vec<_>>>()
}
