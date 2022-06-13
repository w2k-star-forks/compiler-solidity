//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;

use crate::solc::combined_json::CombinedJson;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
use crate::solc::standard_json::output::Output as StandardJsonOutput;

use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug, Default)]
pub struct Build {
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
}

impl Build {
    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        output_assembly: bool,
        output_binary: bool,
        output_abi: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        for (_path, contract) in self.contracts.into_iter() {
            contract.write_to_directory(
                output_directory,
                output_assembly,
                output_binary,
                output_abi,
                overwrite,
            )?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(self, combined_json: &mut CombinedJson) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            let combined_json_contract = combined_json
                .contracts
                .iter_mut()
                .find_map(|(json_path, contract)| {
                    if path.ends_with(json_path) {
                        Some(contract)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Contract `{}` not found in the project", path))?;

            contract.write_to_combined_json(combined_json_contract)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        mut self,
        standard_json: &mut StandardJsonOutput,
    ) -> anyhow::Result<()> {
        let contracts = match standard_json.contracts.as_mut() {
            Some(contracts) => contracts,
            None => return Ok(()),
        };

        for (path, contracts) in contracts.iter_mut() {
            for (name, contract) in contracts.iter_mut() {
                let full_name = format!("{}:{}", path, name);

                if let Some(contract_data) = self.contracts.remove(full_name.as_str()) {
                    let deploy_bytecode = hex::encode(contract_data.deploy_build.bytecode);
                    let runtime_bytecode = hex::encode(contract_data.runtime_build.bytecode);

                    contract.ir_optimized = None;
                    contract.evm = Some(StandardJsonOutputContractEVM::new_zkevm_bytecode(
                        deploy_bytecode,
                        runtime_bytecode,
                    ));

                    contract.deploy_hash = Some(contract_data.deploy_build.hash);
                    contract.deploy_factory_dependencies =
                        Some(contract_data.deploy_build.factory_dependencies);

                    contract.runtime_hash = Some(contract_data.runtime_build.hash);
                    contract.runtime_factory_dependencies =
                        Some(contract_data.runtime_build.factory_dependencies);
                }
            }
        }

        Ok(())
    }
}
