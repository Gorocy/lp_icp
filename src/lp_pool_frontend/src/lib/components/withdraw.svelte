<script>
    import { backend } from "$lib/canisters";

    // standard for most token
    const precision = 1000000;

    let stakenTokenResult = "";
    let tokenResult = "";

    function Withdraw(event) {
        const amount = parseInt(event.target.amount.value * precision);
        console.log(amount);
        backend
        .remove_liquidity_from_pool(amount)
        .then((response) => {
            console.log("Response:", response);

            if (response.Ok !== undefined) {
                const [token, stToken] = response.Ok;

                tokenResult = parseInt(token) / precision;
                stakenTokenResult = parseInt(stToken) / precision;
            } else if (response.Err !== undefined) {
                const error = JSON.stringify(response.Err);
                tokenResult = "Error: " + error;
                stakenTokenResult = "Error: " + error;
            } else {
                tokenResult = "Unexpected response structure";
                stakenTokenResult = "Unexpected response structure";
            }
        })
        .catch((error) => {
            console.error("Error:", error);

            if (error && typeof error === "object") {
                const errorMsg = JSON.stringify(error);
                tokenResult = "Error: " + errorMsg;
                stakenTokenResult = "Error: " + errorMsg;
            } else {
                tokenResult = "Unknown error occurred";
                stakenTokenResult = "Unknown error occurred";
            }
        });
    }
</script>

<div class="container">
    <form on:submit|preventDefault={Withdraw}>
        <label for="name">Withdraw: &nbsp;</label>
        <input id="amount" alt="Name" type="number" step="any" />
        <button type="submit">Click Me!</button>
    </form>
    <p>Received token: {tokenResult}</p>
    <p>Received staken token: {stakenTokenResult}</p>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: 90%;
        border: 5px solid red;
        border-radius: 5px;
    }

    .container p {
        padding: 10px;
        border: 1px solid black;
        border-radius: 5px;
        background-color: #f5f5f5;
    }
</style>
