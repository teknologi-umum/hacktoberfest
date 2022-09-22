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
        Don't know where to start your Hacktoberfest journey? Start exploring our repositories!
      </p>
    </header>
  );
});
