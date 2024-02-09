package playground;

import java.nio.file.Path;

import com.partisiablockchain.BlockchainAddress;
import com.partisiablockchain.language.abicodegen.Multiply;
import com.partisiablockchain.language.junit.ContractBytes;
import com.partisiablockchain.language.junit.ContractTest;
import com.partisiablockchain.language.junit.JunitContractTest;

public final class ZkMultiplyTest extends JunitContractTest {

  private static final ContractBytes CONTRACT_BYTES = ContractBytes.fromPaths(
      Path.of("../target/wasm32-unknown-unknown/release/multiply.zkwa"),
      Path.of("../target/wasm32-unknown-unknown/release/multiply.abi")
  );
  private BlockchainAddress account1;
  private BlockchainAddress zkContract;

  @ContractTest
  public void deployZkContract() {
    account1 = blockchain.newAccount(2);

    byte[] initialize = Multiply.initialize();

    zkContract = blockchain.deployZkContract(account1, CONTRACT_BYTES, initialize);
  }

  @ContractTest(previous = "deployZkContract")
  public void runZkComputation() {
    byte[] guess = Multiply.multiply();
    blockchain.sendAction(account1, zkContract, guess);
  }
}
