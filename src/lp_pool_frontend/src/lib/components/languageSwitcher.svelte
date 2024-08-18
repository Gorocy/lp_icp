<script>
    import { language } from "$lib/stores/language";
  
    function setLanguage(lang) {
      language.set(lang);
    }
  </script>
  
  <div>
    <button on:click={() => setLanguage('en')}>English</button>
    <button on:click={() => setLanguage('pl')}>Polski</button>
  </div>
  