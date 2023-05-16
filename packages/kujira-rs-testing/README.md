Mock test interfaces for [kujira-rs](https://github.com/Team-Kujira/kujira-rs)

Add to dev-dependecies

```toml
[dev-dependencies]
kujira-rs-testing = { git = "https://github.com/team-kujira/kujira-rs-testing" }
```

# mock.rs

This creates a mock interface to the chain core, supporting Oracle queries and Denom msgs and queries.

n.b: minting tokens is not currently supported. current advice is to mock by pre-loading the destination with tokens in test setup, then checking for the correct `mint` events.

Update oracle price

```rs
    app.init_modules(|router, _api, _storage| {
        router
            .custom
            .set_oracle_price(Decimal::from_ratio(1325u128, 100u128));
    });
```

Checking mint events

```rs
    let mint_event = res.events.iter().find(|e| e.ty == "mint").unwrap();
    assert_eq!(mint_event.attributes[0], attr("amount", "8122"));
    assert_eq!(mint_event.attributes[1], attr("denom", STABLE));
    assert_eq!(
        mint_event.attributes[2],
        attr("recipient", contract.clone())
    );
```

# fin.rs

Use these mock contract interfaces to add to `mock::mock_app`

```rs
    let fin_code = ContractWrapper::new(
        kujira_stable_testing::fin::execute,
        kujira_stable_testing::fin::instantiate,
        kujira_stable_testing::fin::query,
    );
    let fin_code_id = app.store_code(Box::new(fin_code));
    let fin_contract = app
        .instantiate_contract(
            fin_code_id,
            owner.clone(),
            &kujira::fin::InstantiateMsg {
                owner: Addr::unchecked("owner"),
                denoms: [
                    Denom::Native(COLLATERAL.to_string()),
                    Denom::Native(STABLE.to_string()),
                ],
                decimal_delta: None,
                price_precision: Precision::DecimalPlaces(3),
            },
            &vec![],
            "fin",
            Some(owner.to_string()),
        )
        .unwrap();
```

# orca.rs

Similarly for Orca.rs

```rs
    let orca_code = ContractWrapper::new(
        kujira_stable_testing::orca::execute,
        kujira_stable_testing::orca::instantiate,
        kujira_stable_testing::orca::query,
    );
    let orca_code_id = app.store_code(Box::new(orca_code));
    let orca_contract = app
        .instantiate_contract(
            orca_code_id,
            owner.clone(),
            &kujira::orca::InstantiateMsg {
                owner: Addr::unchecked("owner"),
                market: Addr::unchecked("market"),
                bid_denom: Denom::Native(STABLE.to_string()),
                collateral_denom: Denom::Cw20(Addr::unchecked(COLLATERAL)),
                bid_threshold: Uint128::from(1000000u128),
                max_slot: 20,
                premium_rate_per_slot: Decimal::from_ratio(1u128, 100u128),
                waiting_period: 600,
                liquidation_fee: Decimal::from_ratio(1u128, 100u128),
                withdrawal_fee: Decimal::from_ratio(1u128, 200u128),
                fee_address: fee_address(),
            },
            &vec![],
            "orca",
            Some(owner.to_string()),
        )
        .unwrap();
```
