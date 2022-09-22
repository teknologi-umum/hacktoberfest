import { component$, useStylesScoped$ } from "@builder.io/qwik";
import styles from "./header.css?inline";

export default component$(() => {
  useStylesScoped$(styles);

  return (
    <header>
      <div class="header">
        <h1 class="header__text">
          TEKNOLOGI UMUM <br />
          HACKTOBERFEST
        </h1>
      </div>
      <p class="detail">
        Bingung Hacktoberfest mau kontribusi kemana? Kuy, mulai kontribusi dari repo punya teknum!
      </p>
    </header>
  );
});
