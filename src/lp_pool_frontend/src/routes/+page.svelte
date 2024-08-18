<script>
  import "../index.scss";
  import Assets from "$lib/components/assets.svelte";
  import Deposit from "$lib/components/deposit.svelte";
  import Swap from "$lib/components/swap.svelte";
  import Withdraw from "$lib/components/withdraw.svelte";
  import AboutEn from "$lib/components/aboutEn.svelte";
  import AboutPl from "$lib/components/aboutPl.svelte";

  let language = "en";

  function switchLanguage(lang) {
    language = lang;
  }
</script>

<div>
  <h1>Liquidity Pool Simulator</h1>
  <div class="container">
    <div class="left">
      <Assets />
    </div>

    <div class="right wrapper">
      <Deposit />
      <Swap />
      <Withdraw />
    </div>
  </div>
  <div>
    <div>
      <button on:click={() => switchLanguage('en')}>English</button>
      <button on:click={() => switchLanguage('pl')}>Polski</button>
    </div>
    {#if language === "en"}
      <AboutEn />
    {:else if language === "pl"}
      <AboutPl />
    {/if}
  </div>
</div>


<style>
  .container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    width: 100vw;
    height: 100vh;
  }

  .wrapper {
    display: flex;
    flex-direction: column;
    gap: 20px;
    justify-content: center;
    align-items: center;
  }

  .left {
    margin: 2vw;
    border: 5px solid rebeccapurple;
    border-radius: 15px;
    /* background-color: rgb(10, 94, 122); */
  }

  .right {
    margin: 2vw;
    border: 5px solid red;
    border-radius: 10px;
    justify-content: center;
    align-items: center;
  }

  .about-section {
    margin-top: 20px;
    text-align: center;
    width: 100%;
  }
</style>
