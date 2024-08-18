<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";

    // standard for most token
    const precision = 1000000;

    let price = "";
    let tokenAmount = "";
    let tokenStAmount = "";
    let tokenLpAmount = "";
    let target = "";

    function lpPrice() {
        backend.lp_price().then((response) => {
            price = parseFloat(response) / precision;
        });
        return false;
    }

    function lpTokenAmount() {
        backend.lp_token_amount().then((response) => {
            tokenAmount = parseFloat(response) / precision;
        });
        return false;
    }

    function lpTokenStAmount() {
        backend.lp_st_token_amount().then((response) => {
            tokenStAmount = parseFloat(response) / precision;
        });
        return false;
    }

    function lpTokenLpAmount() {
        backend.lp_lp_token_amount().then((response) => {
            tokenLpAmount = parseFloat(response) / precision;
        });
        return false;
    }

    function lpTarget() {
        backend.lp_target_amount().then((response) => {
            target = parseFloat(response) / precision;
        });
        return false;
    }
    onMount(() => {
        lpPrice();
        lpTokenAmount();
        lpTokenStAmount();
        lpTokenLpAmount();
        lpTarget();
    });
</script>

<div class="wrapper">
    <div class="container">
        <p>Price: {price}</p>
        <p>Token amount: {tokenAmount}</p>
        <p>Staken token amount: {tokenStAmount}</p>
        <p>Liquidity pool token amount: {tokenLpAmount}</p>
        <p>Target: {target}</p>
    </div>
</div>

<style>
    .wrapper {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }
    .container {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: fit-content;
    }

    .container p {
        padding: 10px;
        border: 1px solid black;
        border-radius: 5px;
        background-color: #f5f5f5;
    }
</style>
