<script>
    import { backend } from "$lib/canisters";

    // standard for most token
    const precision = 1000000;

    let swapResult = "";

    function Swap(event) {
        const amount = parseInt(event.target.amount.value * precision);
        console.log(amount);
        backend
            .swap_st_pool(amount)
            .then((response) => {
                if (response.Ok !== undefined) {
                    swapResult = parseInt(response.Ok) / precision;
                } else if (response.Err !== undefined) {
                    swapResult = "Error: " + JSON.stringify(response.Err);
                } else {
                    swapResult = "Unexpected response structure";
                }
            })
            .catch((error) => {
                if (error && typeof error === "object") {
                    swapResult = JSON.stringify(error);
                } else {
                    swapResult = "Unknown error occurred";
                }
            });
    }
</script>

<div class="container">
    <form on:submit|preventDefault={Swap}>
        <label for="name">Swap staken: &nbsp;</label>
        <input id="amount" alt="Name" type="number" step="any" />
        <button type="submit">Click Me!</button>
    </form>
    <p>Recived: {swapResult}</p>
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
