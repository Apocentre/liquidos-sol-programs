import * as anchor from "@coral-xyz/anchor";
import {
  Raydium,
  CREATE_CPMM_POOL_PROGRAM, DEVNET_PROGRAM_ID,
  CurveCalculator,
  FeeOn,
} from "@raydium-io/raydium-sdk-v2";

const VALID_PROGRAM_ID = new Set([
  CREATE_CPMM_POOL_PROGRAM.toBase58(),
  DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM.toBase58(),
])

export const isValidCpmm = (id) => VALID_PROGRAM_ID.has(id)

const {BN} = anchor.default;

class RaydiumHelper {
  async create(connection, cluster) {
    this.raydium = await Raydium.load({connection, cluster});
  }

  async getPoolInfo(poolId) {
    let poolInfo;
    let poolKeys;
    let rpcData;

    if (this.raydium.cluster === 'mainnet') {
      // if you wish to get pool info from rpc, also can modify logic to go rpc method directly
      const data = await this.raydium.api.fetchPoolById({ ids: poolId })
      poolInfo = data[0];
      if (!isValidCpmm(poolInfo.programId)) throw new Error('target pool is not CPMM pool')
      rpcData = await this.raydium.cpmm.getRpcPoolInfo(poolInfo.id, true)
    } else {
      const data = await this.raydium.cpmm.getPoolInfoFromRpc(poolId)
      poolInfo = data.poolInfo
      poolKeys = data.poolKeys
      rpcData = data.rpcData
    }

    return {poolInfo, poolKeys, rpcData}
  }

  /// * poolID - the pool state account
  /// * inputMint - the token we are selling
  /// * slippage - 0 - 1 where 1 is 100%
  async getSwapBaseInResult(poolId, inputMint, inputAmount, slippage) {
    const {poolInfo, rpcData} = await this.getPoolInfo(poolId);

    const baseIn = inputMint.toBase58() === poolInfo.mintA.address
    // swap pool mintA for mintB
    const swapResult = CurveCalculator.swapBaseInput(
      inputAmount,
      baseIn ? rpcData.baseReserve : rpcData.quoteReserve,
      baseIn ? rpcData.quoteReserve : rpcData.baseReserve,
      rpcData.configInfo.tradeFeeRate,
      rpcData.configInfo.creatorFeeRate,
      rpcData.configInfo.protocolFeeRate,
      rpcData.configInfo.fundFeeRate,
      rpcData.feeOn === FeeOn.BothToken || rpcData.feeOn === FeeOn.OnlyTokenB,
    )

    swapResult.outputAmount = swapResult.outputAmount.mul(new BN((1 - slippage) * 10000)).div(new BN(10000));
    return swapResult
  }

  /// * poolID - the pool state account
  /// * outputMint - the token we are buying
  /// * slippage - 0 - 1 where 1 is 100%
  async getSwapBaseOutResult(poolId, outputMint, outputAmount, slippage) {
    const {poolInfo, rpcData} = await this.getPoolInfo(poolId)
    const baseIn = outputMint.toBase58() === poolInfo.mintB.address
    // swap pool mintA for mintB
    const swapResult = CurveCalculator.swapBaseOutput(
      outputAmount.gt(rpcData[baseIn ? 'quoteReserve' : 'baseReserve'])
        ? rpcData[baseIn ? 'quoteReserve' : 'baseReserve'].sub(new BN(1))
        : outputAmount,
      baseIn ? rpcData.baseReserve : rpcData.quoteReserve,
      baseIn ? rpcData.quoteReserve : rpcData.baseReserve,
      rpcData.configInfo.tradeFeeRate,
      rpcData.configInfo.creatorFeeRate,
      rpcData.configInfo.protocolFeeRate,
      rpcData.configInfo.fundFeeRate,
      rpcData.feeOn === FeeOn.BothToken || rpcData.feeOn === FeeOn.OnlyTokenB
    )

    swapResult.inputAmount = swapResult.inputAmount.mul(new BN((1 + slippage) * 10000)).div(new BN(10000));
    return swapResult
  }
}

export default RaydiumHelper;
