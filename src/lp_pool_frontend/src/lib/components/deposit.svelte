<script>
    import { backend } from "$lib/canisters";

    // standard for most token
    const precision = 1000000;

    let depositResult = "";

    function Deposit(event) {
        const amount = parseInt(event.target.amount.value * precision);
        console.log(amount);
        backend
            .add_liquidity_to_pool(amount)
            .then((response) => {
                if (response.Ok !== undefined) {
                    depositResult = parseInt(response.Ok) / precision;
                } else if (response.Err !== undefined) {
                    depositResult = "Error: " + JSON.stringify(response.Err);
                } else {
                    depositResult = "Unexpected response structure";
                }
            })
            .catch((error) => {
                if (error && typeof error === "object") {
                    depositResult = JSON.stringify(error);
                } else {
                    depositResult = "Unknown error occurred";
                }
            });
    }
</script>

<div class="container">
    <form on:submit|preventDefault={Deposit}>
        <label for="name">Deposit: &nbsp;</label>
        <input id="amount" alt="Name" type="number" step="any" />
        <button type="submit">Click Me!</button>
    </form>
    <p>Recived: {depositResult}</p>
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
