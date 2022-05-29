mod utils;

const EXCHANGE_NAME: &str = "binance";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616176861895,"s":"BTCUSDT","a":640283266,"p":"58942.01000000","q":"0.00035600","f":716849523,"l":716849523,"T":1616176861893,"m":false,"M":true}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616176861895,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00035600);
        assert_eq!(trade.quantity_quote, 0.00035600 * 58942.01);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_210625@aggTrade","data":{"e":"aggTrade","E":1616201787561,"a":5091038,"s":"BTCUSD_210625","p":"62838.0","q":"5","f":7621250,"l":7621250,"T":1616201787407,"m":true}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616201787561,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 500.0 / 62838.0);
        assert_eq!(trade.quantity_quote, 500.0);
        assert_eq!(trade.quantity_contract, Some(5.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"btcusdt_210625@aggTrade","data":{"e":"aggTrade","E":1616201036113,"a":21021,"s":"BTCUSDT_210625","p":"62595.8","q":"0.094","f":21824,"l":21824,"T":1616201035958,"m":false}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616201036113,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.094);
        assert_eq!(trade.quantity_quote, 0.094 * 62595.8);
        assert_eq!(trade.quantity_contract, Some(0.094));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@aggTrade","data":{"e":"aggTrade","E":1616201883458,"a":41045788,"s":"BTCUSD_PERP","p":"58570.1","q":"58","f":91864326,"l":91864327,"T":1616201883304,"m":true}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616201883458,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 58570.1);
        assert_eq!(trade.quantity_base, 5800.0 / 58570.1);
        assert_eq!(trade.quantity_quote, 5800.0);
        assert_eq!(trade.quantity_contract, Some(58.0));

        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616202009196,"a":389551486,"s":"BTCUSDT","p":"58665.00","q":"0.043","f":621622993,"l":621622993,"T":1616202009188,"m":false}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616202009196,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.043);
        assert_eq!(trade.quantity_quote, 0.043 * 58665.00);
        assert_eq!(trade.quantity_contract, Some(0.043));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    #[ignore]
    fn option() {
        let raw_msg = r#"{"stream":"BTCUSDT_C@TRADE_ALL","data":{"e":"trade_all","E":1616205287778,"s":"BTCUSDT_C","t":[{"t":"315","p":"4842.24","q":"0.0001","b":"4612047757752932782","a":"4612057653433061439","T":1616204382000,"s":"1","S":"BTC-210430-68000-C"},{"t":"805","p":"5616.36","q":"0.0001","b":"4612047757752932781","a":"4612057653433055969","T":1616204357000,"s":"1","S":"BTC-210430-64000-C"},{"t":"313","p":"7028.44","q":"0.0001","b":"4612015871915728334","a":"4612057653433051715","T":1616204344000,"s":"1","S":"BTC-210430-60000-C"}]}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::EuropeanOption,
                "BTC/USDT".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616205287778,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.0001);
        assert_eq!(trades[0].quantity_quote, 0.0001 * 4842.24);
        assert_eq!(trades[0].quantity_contract, Some(0.0001));
    }
}

#[cfg(test)]
mod funding_rate {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_funding_rate};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@markPrice","data":{"e":"markPriceUpdate","E":1617309477000,"s":"BTCUSD_PERP","p":"59012.56007222","P":"58896.00503145","r":"0.00073689","T":1617321600000}}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTCUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1617309477000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00073689);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        let raw_msg = r#"{"stream":"!markPrice@arr","data":[{"e":"markPriceUpdate","E":1617309501002,"s":"BTCUSD_PERP","p":"59003.37984561","P":"58896.41602208","r":"0.00073684","T":1617321600000},{"e":"markPriceUpdate","E":1617309501002,"s":"ETHUSD_PERP","p":"1981.89000000","P":"1975.18948029","r":"0.00100944","T":1617321600000}]}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00073684);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        assert_eq!(funding_rates[1].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[1].funding_rate, 0.00100944);
        assert_eq!(funding_rates[1].funding_time, 1617321600000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@markPrice","data":{"e":"markPriceUpdate","E":1617308820003,"s":"BTCUSDT","p":"58940.14924532","P":"58905.14663658","i":"58857.26693664","r":"0.00058455","T":1617321600000}}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1617308820003,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00058455);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        let raw_msg = r#"{"stream":"!markPrice@arr","data":[{"e":"markPriceUpdate","E":1617309024002,"s":"BTCUSDT","p":"59022.53514719","P":"58902.34482833","i":"58936.68384000","r":"0.00058959","T":1617321600000},{"e":"markPriceUpdate","E":1617309024002,"s":"ETHUSDT","p":"1981.15704420","P":"1974.79557094","i":"1978.08197502","r":"0.00059142","T":1617321600000}]}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00058959);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        assert_eq!(funding_rates[1].pair, "ETH/USDT".to_string());
        assert_eq!(funding_rates[1].funding_rate, 0.00059142);
        assert_eq!(funding_rates[1].funding_time, 1617321600000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1622363903670,"s":"BTCUSDT","U":11294093710,"u":11294093726,"b":[["35743.98000000","0.00000000"],["35743.87000000","0.00001500"]],"a":[["35743.88000000","0.24000000"],["35743.97000000","0.00000000"]]}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622363903670,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622363903670);
        assert_eq!(orderbook.seq_id, Some(11294093726));

        assert_eq!(orderbook.bids[0].price, 35743.98);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[1].price, 35743.87);
        assert_eq!(orderbook.bids[1].quantity_base, 0.000015);
        assert_eq!(orderbook.bids[1].quantity_quote, 35743.87 * 0.000015);

        assert_eq!(orderbook.asks[0].price, 35743.88);
        assert_eq!(orderbook.asks[0].quantity_base, 0.24);
        assert_eq!(orderbook.asks[0].quantity_quote, 35743.88 * 0.24);

        assert_eq!(orderbook.asks[1].price, 35743.97);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_210625@depth@100ms","data":{"e":"depthUpdate","E":1622368000245,"T":1622368000234,"s":"BTCUSD_210625","ps":"BTCUSD","U":127531213607,"u":127531214406,"pu":127531213513,"b":[["35943.8","60"],["35965.2","896"]],"a":[["36038.3","9"],["36038.4","21"]]}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622368000245,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622368000234);
        assert_eq!(orderbook.seq_id, Some(127531214406));
        assert_eq!(orderbook.prev_seq_id, Some(127531213513));

        assert_eq!(orderbook.bids[0].price, 35943.8);
        assert_eq!(orderbook.bids[0].quantity_base, 6000.0 / 35943.8);
        assert_eq!(orderbook.bids[0].quantity_quote, 6000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 60.0);

        assert_eq!(orderbook.bids[1].price, 35965.2);
        assert_eq!(orderbook.bids[1].quantity_base, 89600.0 / 35965.2);
        assert_eq!(orderbook.bids[1].quantity_quote, 89600.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 896.0);

        assert_eq!(orderbook.asks[0].price, 36038.3);
        assert_eq!(orderbook.asks[0].quantity_base, 900.0 / 36038.3);
        assert_eq!(orderbook.asks[0].quantity_quote, 900.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 9.0);

        assert_eq!(orderbook.asks[1].price, 36038.4);
        assert_eq!(orderbook.asks[1].quantity_base, 2100.0 / 36038.4);
        assert_eq!(orderbook.asks[1].quantity_quote, 2100.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 21.0);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"ethusdt_210625@depth@100ms","data":{"e":"depthUpdate","E":1622368962075,"T":1622368962065,"s":"ETHUSDT_210625","U":475700780918,"u":475700783070,"pu":475700774972,"b":[["2437.04","82.320"],["2437.07","0.000"]],"a":[["2441.23","1.500"],["2441.24","0.220"]]}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "ETH/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622368962075,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622368962065);
        assert_eq!(orderbook.seq_id, Some(475700783070));
        assert_eq!(orderbook.prev_seq_id, Some(475700774972));

        assert_eq!(orderbook.bids[0].price, 2437.04);
        assert_eq!(orderbook.bids[0].quantity_base, 82.32);
        assert_eq!(orderbook.bids[0].quantity_quote, 2437.04 * 82.32);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 82.32);

        assert_eq!(orderbook.bids[1].price, 2437.07);
        assert_eq!(orderbook.bids[1].quantity_base, 0.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[0].price, 2441.23);
        assert_eq!(orderbook.asks[0].quantity_base, 1.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 2441.23 * 1.5);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.5);

        assert_eq!(orderbook.asks[1].price, 2441.24);
        assert_eq!(orderbook.asks[1].quantity_base, 0.220);
        assert_eq!(orderbook.asks[1].quantity_quote, 2441.24 * 0.220);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.220);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1622371244693,"T":1622371244687,"s":"BTCUSDT","U":475776377463,"u":475776380184,"pu":475776377452,"b":[["35729.77","1.600"],["35750.00","5.106"]],"a":[["35819.20","0.211"],["35820.31","0.001"]]}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622371244693,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622371244687);
        assert_eq!(orderbook.seq_id, Some(475776380184));
        assert_eq!(orderbook.prev_seq_id, Some(475776377452));

        assert_eq!(orderbook.bids[0].price, 35729.77);
        assert_eq!(orderbook.bids[0].quantity_base, 1.6);
        assert_eq!(orderbook.bids[0].quantity_quote, 35729.77 * 1.6);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1.6);

        assert_eq!(orderbook.bids[1].price, 35750.0);
        assert_eq!(orderbook.bids[1].quantity_base, 5.106);
        assert_eq!(orderbook.bids[1].quantity_quote, 35750.0 * 5.106);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 5.106);

        assert_eq!(orderbook.asks[0].price, 35819.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.211);
        assert_eq!(orderbook.asks[0].quantity_quote, 35819.2 * 0.211);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.211);

        assert_eq!(orderbook.asks[1].price, 35820.31);
        assert_eq!(orderbook.asks[1].quantity_base, 0.001);
        assert_eq!(orderbook.asks[1].quantity_quote, 35820.31 * 0.001);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.001);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1622370862564,"T":1622370862553,"s":"BTCUSD_PERP","ps":"BTCUSD","U":127559587191,"u":127559588177,"pu":127559587113,"b":[["35365.9","1400"],["35425.8","561"]],"a":[["35817.8","7885"],["35818.7","307"]]}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622370862564,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622370862553);
        assert_eq!(orderbook.seq_id, Some(127559588177));
        assert_eq!(orderbook.prev_seq_id, Some(127559587113));

        assert_eq!(orderbook.bids[0].price, 35365.9);
        assert_eq!(orderbook.bids[0].quantity_base, 140000.0 / 35365.9);
        assert_eq!(orderbook.bids[0].quantity_quote, 140000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1400.0);

        assert_eq!(orderbook.bids[1].price, 35425.8);
        assert_eq!(orderbook.bids[1].quantity_base, 56100.0 / 35425.8);
        assert_eq!(orderbook.bids[1].quantity_quote, 56100.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 561.0);

        assert_eq!(orderbook.asks[0].price, 35817.8);
        assert_eq!(orderbook.asks[0].quantity_base, 788500.0 / 35817.8);
        assert_eq!(orderbook.asks[0].quantity_quote, 788500.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 7885.0);

        assert_eq!(orderbook.asks[1].price, 35818.7);
        assert_eq!(orderbook.asks[1].quantity_base, 30700.0 / 35818.7);
        assert_eq!(orderbook.asks[1].quantity_quote, 30700.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 307.0);
    }

    #[test]
    fn option() {}
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"ethusdt@depth20","data":{"lastUpdateId":17044571457,"bids":[["1782.00000000","6.48300000"],["1781.95000000","0.03000000"]],"asks":[["1782.01000000","15.46080000"],["1782.02000000","0.00780000"]]}}"#;

        assert_eq!(
            "ETHUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_220624@depth20","data":{"e":"depthUpdate","E":1653817158954,"T":1653817158700,"s":"BTCUSD_220624","ps":"BTCUSD","U":462164213658,"u":462164213658,"pu":462164212205,"b":[["29045.1","254"],["29041.7","49"]],"a":[["29045.2","247"],["29047.4","26"]]}}"#;

        assert_eq!(
            "BTCUSD_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653817158954,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"ethusdt_220624@depth20","data":{"e":"depthUpdate","E":1653817303425,"T":1653817303352,"s":"ETHUSDT_220624","U":1553394839728,"u":1553394845873,"pu":1553394837460,"b":[["1782.82","0.254"],["1782.80","0.678"]],"a":[["1783.20","0.091"],["1783.21","0.556"]]}}"#;

        assert_eq!(
            "ETHUSDT_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653817303425,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@depth20","data":{"e":"depthUpdate","E":1653817423925,"T":1653817423920,"s":"BTCUSD_PERP","ps":"BTCUSD","U":462165801124,"u":462165806312,"pu":462165801122,"b":[["28955.8","2500"],["28955.7","1"]],"a":[["28955.9","16261"],["28956.0","330"]]}}"#;

        assert_eq!(
            "BTCUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653817423925,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"ethusdt@depth20","data":{"e":"depthUpdate","E":1651122265861,"T":1651122265854,"s":"ETHUSDT","U":1437010873371,"u":1437010882721,"pu":1437010873329,"b":[["2886.71","0.454"],["2886.70","2.755"],["2886.67","1.000"]],"a":[["2886.72","77.215"],["2886.73","1.734"],["2886.74","0.181"]]}}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2TopK,
            "ETH/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1651122265861,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1651122265854);
        assert_eq!(orderbook.seq_id, Some(1437010882721));
        assert_eq!(orderbook.prev_seq_id, Some(1437010873329));

        assert_eq!(orderbook.bids[0].price, 2886.71);
        assert_eq!(orderbook.bids[0].quantity_base, 0.454);
        assert_eq!(orderbook.bids[0].quantity_quote, 2886.71 * 0.454);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.454);

        assert_eq!(orderbook.bids[2].price, 2886.67);
        assert_eq!(orderbook.bids[2].quantity_base, 1.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 2886.67 * 1.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 1.000);

        assert_eq!(orderbook.asks[0].price, 2886.72);
        assert_eq!(orderbook.asks[0].quantity_base, 77.215);
        assert_eq!(orderbook.asks[0].quantity_quote, 2886.72 * 77.215);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 77.215);

        assert_eq!(orderbook.asks[2].price, 2886.74);
        assert_eq!(orderbook.asks[2].quantity_base, 0.181);
        assert_eq!(orderbook.asks[2].quantity_quote, 2886.74 * 0.181);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.181);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"!bookTicker","data":{"u":19575390521,"s":"BTCUSDT","b":"29010.90000000","B":"13.94302000","a":"29010.91000000","A":"3.99953000"}}"#;

        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let raw_msg = r#"{"stream":"btcusdt@bookTicker","data":{"u":19576530692,"s":"BTCUSDT","b":"29008.97000000","B":"8.19442000","a":"29008.98000000","A":"5.32985000"}}"#;

        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"!bookTicker","data":{"u":462118190224,"e":"bookTicker","s":"XMRUSD_PERP","ps":"XMRUSD","b":"172.74","B":"86","a":"172.81","A":"25","T":1653811915499,"E":1653811915502}}"#;

        assert_eq!(
            "XMRUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653811915502,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"!bookTicker","data":{"e":"bookTicker","u":1553205153844,"s":"BALUSDT","b":"6.718","B":"16.2","a":"6.719","A":"3.7","T":1653812037547,"E":1653812037552}}"#;

        assert_eq!(
            "BALUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653812037552,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"!bookTicker","data":{"u":462119409422,"e":"bookTicker","s":"GMTUSD_PERP","ps":"GMTUSD","b":"0.93900","B":"499","a":"0.94100","A":"632","T":1653812104216,"E":1653812104221}}"#;

        assert_eq!(
            "GMTUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653812104221,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"ethusd_perp@bookTicker","data":{"u":462169572570,"e":"bookTicker","s":"ETHUSD_PERP","ps":"ETHUSD","b":"1776.25","B":"10779","a":"1776.26","A":"37544","T":1653817930434,"E":1653817930438}}"#;

        assert_eq!(
            "ETHUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653817930438,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"!bookTicker","data":{"e":"bookTicker","u":1553208933856,"s":"MASKUSDT","b":"1.7370","B":"4966","a":"1.7380","A":"11","T":1653812171963,"E":1653812171970}}"#;

        assert_eq!(
            "MASKUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653812171970,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"ethusdt@bookTicker","data":{"e":"bookTicker","u":1553413152520,"s":"ETHUSDT","b":"1778.54","B":"15.164","a":"1778.55","A":"7.289","T":1653817855284,"E":1653817855289}}"#;

        assert_eq!(
            "ETHUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653817855289,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"ethusdt@ticker","data":{"e":"24hrTicker","E":1653812650349,"s":"ETHUSDT","p":"28.23000000","P":"1.600","w":"1781.16275609","x":"1764.61000000","c":"1792.84000000","Q":"0.55720000","b":"1792.83000000","B":"1.62740000","a":"1792.84000000","A":"20.29140000","o":"1764.61000000","h":"1808.98000000","l":"1748.94000000","v":"471703.53110000","q":"840180761.51358100","O":1653726250344,"C":1653812650344,"F":841094172,"L":841646650,"n":552479}}"#;

        assert_eq!(
            "ETHUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653812650349,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"!ticker@arr","data":[{"e":"24hrTicker","E":1653812100450,"s":"ETHBTC","p":"0.00031500","P":"0.515","w":"0.06150587","x":"0.06118800","c":"0.06150300","Q":"0.74000000","b":"0.06150300","B":"18.96220000","a":"0.06150400","A":"10.94010000","o":"0.06118800","h":"0.06221700","l":"0.06079900","v":"116854.45230000","q":"7187.23459814","O":1653725700257,"C":1653812100257,"F":342624389,"L":342741830,"n":117442}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653812100450,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"bnbusd_220624@ticker","data":{"e":"24hrTicker","E":1653814590778,"s":"BNBUSD_220624","ps":"BNBUSD","p":"1.134","P":"0.384","w":"299.21521423","c":"296.672","Q":"2","o":"295.538","h":"305.367","l":"293.303","v":"654348","q":"21868.80776371","O":1653728160000,"C":1653814590774,"F":2127486,"L":2138545,"n":11060}}"#;

        assert_eq!(
            "BNBUSD_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653814590778,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"!ticker@arr","data":[{"e":"24hrTicker","E":1653814699950,"s":"AVAXUSD_PERP","ps":"AVAXUSD","p":"2.60","P":"11.439","w":"24.25184175","c":"25.33","Q":"516","o":"22.73","h":"25.92","l":"22.26","v":"3259077","q":"1343847.21496168","O":1653728280000,"C":1653814699945,"F":9534407,"L":9598591,"n":64185}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653814699950,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"btcusdt_220624@ticker","data":{"e":"24hrTicker","E":1653814783325,"s":"BTCUSDT_220624","p":"196.5","P":"0.679","w":"29043.7","c":"29122.5","Q":"0.004","o":"28926.0","h":"29335.0","l":"28834.4","v":"1047.859","q":"30433679.16","O":1653728340000,"C":1653814783320,"F":2872692,"L":2893528,"n":20837}}"#;

        assert_eq!(
            "BTCUSDT_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653814783325,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"!ticker@arr","data":[{"e":"24hrTicker","E":1653813900353,"s":"BTCUSDT","p":"213.70","P":"0.740","w":"28973.36","c":"29093.90","Q":"0.017","o":"28880.20","h":"29265.70","l":"28755.00","v":"173254.241","q":"5019757418.36","O":1653727500000,"C":1653813900348,"F":2299852977,"L":2301666553,"n":1813560}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653813900353,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@ticker","data":{"e":"24hrTicker","E":1653815020148,"s":"BTCUSD_PERP","ps":"BTCUSD","p":"238.2","P":"0.827","w":"28943.21988874","c":"29031.8","Q":"83","o":"28793.6","h":"29222.8","l":"28728.4","v":"12229247","q":"42252.54497257","O":1653728580000,"C":1653815020145,"F":442530608,"L":442717685,"n":187078}}"#;

        assert_eq!(
            "BTCUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653815020148,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"!ticker@arr","data":[{"e":"24hrTicker","E":1653814800360,"s":"APEUSD_PERP","ps":"APEUSD","p":"-0.0510","P":"-0.827","w":"6.20092494","c":"6.1180","Q":"200","o":"6.1690","h":"6.3870","l":"5.9620","v":"859567","q":"1386191.58913317","O":1653728400000,"C":1653814800354,"F":4337375,"L":4357994,"n":20620}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653814800360,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"ethusdt@ticker","data":{"e":"24hrTicker","E":1653815205866,"s":"ETHUSDT","p":"24.69","P":"1.405","w":"1782.01","c":"1782.02","Q":"0.003","o":"1757.33","h":"1809.33","l":"1750.45","v":"1678290.411","q":"2990736920.72","O":1653728760000,"C":1653815205856,"F":1689354205,"L":1691266554,"n":1912297}}"#;

        assert_eq!(
            "ETHUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653815205866,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        let raw_msg = r#"{"stream":"!ticker@arr","data":[{"e":"24hrTicker","E":1653814800126,"s":"IOTXUSDT","p":"0.00053","P":"1.525","w":"0.03552","c":"0.03529","Q":"861","o":"0.03476","h":"0.03645","l":"0.03465","v":"346584392","q":"12312309.61000","O":1653728400000,"C":1653814800117,"F":94208734,"L":94269527,"n":60794}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653814800126,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@kline_1m","data":{"e":"kline","E":1653818762502,"s":"BTCUSDT","k":{"t":1653818760000,"T":1653818819999,"s":"BTCUSDT","i":"1m","f":1384844002,"L":1384844032,"o":"29038.46000000","c":"29038.47000000","h":"29038.47000000","l":"29038.46000000","v":"0.20926000","n":31,"x":false,"q":"6076.58918320","V":"0.10436000","Q":"3030.45472920","B":"0"}}}"#;

        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653818762502,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_220624@kline_1m","data":{"e":"kline","E":1653818854836,"s":"BTCUSD_220624","k":{"t":1653818820000,"T":1653818879999,"s":"BTCUSD_220624","i":"1m","f":12373411,"L":12373422,"o":"29105.5","c":"29107.9","h":"29107.9","l":"29096.9","v":"191","n":12,"x":false,"q":"0.65623556","V":"34","Q":"0.11681070","B":"0"}}}"#;

        assert_eq!(
            "BTCUSD_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653818854836,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"btcusdt_220624@kline_1m","data":{"e":"kline","E":1653818905630,"s":"BTCUSDT_220624","k":{"t":1653818880000,"T":1653818939999,"s":"BTCUSDT_220624","i":"1m","f":2894511,"L":2894528,"o":"29135.9","c":"29141.5","h":"29149.9","l":"29135.9","v":"1.447","n":18,"x":false,"q":"42169.8708","V":"0.241","Q":"7023.0583","B":"0"}}}"#;

        assert_eq!(
            "BTCUSDT_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1653818905630,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"ethusd_perp@kline_1m","data":{"e":"kline","E":1653818962599,"s":"ETHUSD_PERP","k":{"t":1653818940000,"T":1653818999999,"s":"ETHUSD_PERP","i":"1m","f":413873402,"L":413874164,"o":"1786.56","c":"1788.67","h":"1789.24","l":"1785.25","v":"401601","n":763,"x":false,"q":"2246.59444657","V":"254246","Q":"1422.31905487","B":"0"}}}"#;

        assert_eq!(
            "ETHUSD_PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653818962599,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@kline_1M","data":{"e":"kline","E":1653819041520,"s":"BTCUSDT","k":{"t":1651363200000,"T":1654041599999,"s":"BTCUSDT","i":"1M","f":2172726276,"L":2301806561,"o":"37614.40","c":"29075.50","h":"40071.70","l":"26631.00","v":"13431981.671","n":129025447,"x":false,"q":"423075730671.12853","V":"6700065.176","Q":"211000435586.65000","B":"0"}}}"#;

        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653819041520,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}
