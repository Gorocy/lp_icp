<script>
    // Opcjonalnie: Możesz zaimportować dodatkowe style lub komponenty, jeśli są potrzebne.
  </script>
  
  <style>
    .about-section {
      padding: 20px;
      border: 2px solid #ccc;
      border-radius: 10px;
      background-color: #f9f9f9;
      margin-top: 20px;
    }
  
    .about-section h2 {
      color: #333;
      margin-bottom: 10px;
    }
  
    .about-section p {
      margin-bottom: 10px;
      line-height: 1.6;
    }
  
    .about-section code {
      background-color: #eee;
      padding: 2px 4px;
      border-radius: 4px;
    }
  </style>
  
  <div class="about-section">
    <h2>O Implementacji Puli Płynności</h2>
    <p>
      Ta implementacja puli płynności jest napisana w języku Rust i zawiera kluczowe komponenty oraz metody, 
      które symulują zachowanie puli płynności w kontekście zdecentralizowanych finansów (DeFi).
    </p>
    
    <h3>Inicjalizacja</h3>
    <p>
      Funkcja <code>LpPool::init</code> inicjuje pulę płynności z podanymi parametrami: 
      <code>price</code>, <code>min_fee</code>, <code>max_fee</code> oraz <code>liquidity_target</code>. 
      Parametry te są weryfikowane w celu zapewnienia, że:
      <ul>
        <li><code>min_fee</code> jest mniejsza niż <code>max_fee</code>.</li>
        <li><code>price</code> jest większa od 0.</li>
        <li><code>liquidity_target</code> jest większy od 0.</li>
      </ul>
    <!-- </p> -->
  
    <h3>Dodawanie Płynności</h3>
    <p>
      Funkcja <code>add_liquidity</code> pozwala użytkownikom na zdeponowanie tokenów w puli. Funkcja 
      oblicza liczbę tokenów dostarczanych przez dostawcę płynności (LP), które mają zostać wybite na podstawie 
      aktualnej wartości aktywów w puli. Obliczenia te uwzględniają proporcję puli przed i po dokonaniu wpłaty.
    </p>
  
    <h3>Usuwanie Płynności</h3>
    <p>
      Funkcja <code>remove_liquidity</code> umożliwia użytkownikom wycofanie ich udziału w puli. Oblicza ona 
      ilość tokenów i stakowanych tokenów, które należy zwrócić na podstawie proporcji LP tokenów, które są 
      wymieniane. Funkcja zapewnia, że żądana ilość jest prawidłowa i że rezerwy puli są odpowiednio aktualizowane.
    </p>
    <p>
      Ponieważ jest to symulacja, każdy użytkownik ma nielimitowaną ilość tokenów. Jeśli chcesz zrestartować symulację 
      lub zresetować stan puli, wystarczy użyć funkcji <code>remove_liquidity</code> do wycofania wszystkich aktywów.
    </p>
  
    <h3>Wymiana Tokenów</h3>
    <p>
      Funkcja <code>swap</code> pozwala użytkownikom wymieniać stakowane tokeny na zwykłe tokeny. Funkcja stosuje 
      dynamiczną opłatę, obliczaną za pomocą wzoru:
      <code>unstake_fee = max_fee - (max_fee - min_fee) * amount_after / target</code>. 
      Wzór ten dostosowuje opłatę w zależności od ilości pozostałych rezerw puli w stosunku do docelowej płynności.
    </p>
  
    <p>
      Wzór zapewnia, że opłata maleje, gdy pula zbliża się do swojego docelowego poziomu płynności, co zachęca do 
      wymian, które utrzymują równowagę puli.
    </p>
  
    <h3>Dodatkowe Uwagi</h3>
    <p>
      Kod zawiera różne mechanizmy kontroli, aby zapewnić, że operacje takie jak dodawanie płynności, 
      usuwanie płynności i wymiana tokenów są wykonywane bezpiecznie i w ramach parametrów puli. 
      Ta implementacja może zostać rozszerzona lub zmodyfikowana, aby wspierać dodatkowe funkcje lub specyficzne 
      przypadki użycia w DeFi.
    </p>
    <p>
      Należy pamiętać, że jest to symulowane środowisko, a rzeczywiste warunki mogą się różnić. 
      Nielimitowana podaż tokenów dla użytkowników jest specyficzna dla tego scenariusza testowego.
    </p>
  </div>
  