### ethers-rs Etherscan API issue

An issue with etherscan API while using `ethers@2.0.2` crate.

### Steps to Reproduce

1. Clone the repo: `git clone https://github.com/shekohex/ethers-etherscan-issue.git`
2. Set `ETHERSCAN_API_KEY` env variable with your etherscan API key. (for our case, use Polygon API key)
3. Set `CHAIN` env variable with the chain you want to use (for our case `polygon`).
4. Run `cargo run`

### Expected Behavior

The program should print the Gas Oracle data.

### Actual Behavior

**It Returns an error**

```rs
  ERROR etherscan: Failed to deserialize response: data did not match any variant of untagged enum ResponseData, res: "{\"status\":\"1\",\"message\":\"OK\",\"result\":{\"LastBlock\":\"41171167\",\"SafeGasPrice\":\"119.9\",\"ProposeGasPrice\":\"141.9\",\"FastGasPrice\":\"142.9\",\"suggestBaseFee\":\"89.82627877\",\"gasUsedRatio\":\"0.399191166666667,0.4847166,0.997667533333333,0.538075133333333,0.343416033333333\",\"UsdPrice\":\"1.15\"}}"
    at /home/shady/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ethers-etherscan-2.0.2/src/lib.rs:205

   INFO ethers_etherscan_issue: Gas Oracle Info, gas_oracle: Err(Serde(Error("data did not match any variant of untagged enum ResponseData", line: 0, column: 0)))
    at src/main.rs:39
```

**JSON Response**

```json
{
  "status": "1",
  "message": "OK",
  "result": {
    "LastBlock": "41171167",
    "SafeGasPrice": "119.9",
    "ProposeGasPrice": "141.9",
    "FastGasPrice": "142.9",
    "suggestBaseFee": "89.82627877",
    "gasUsedRatio": "0.399191166666667,0.4847166,0.997667533333333,0.538075133333333,0.343416033333333",
    "UsdPrice": "1.15"
  }
}
```

### Environment

- Operating System: `Linux workstation 5.15.104-2-MANJARO #1 SMP PREEMPT Thu Mar 23 01:40:42 UTC 2023 x86_64 GNU/Linux`
- Ethers Version: `2.0.2`
