import { component$, Slot } from "@builder.io/qwik";
import "@fontsource/cairo/500.css";
import "@fontsource/cairo/600.css";
import "@fontsource/press-start-2p/400.css";

export default component$(() => {
  return (
    <>
      <div class="starfield" />
      <div class="blue-nebula" />
      <div class="orange-nebula" />
      <div class="pink-nebula" />
      {Array(4)
        .fill(0)
        .map((_, i) => (
          <div class={`line-${i}`} />
        ))}
      <main>
        <Slot />
      </main>
      <footer>
        <span class="footer-text">
          Punya masukan untuk website ini? Gas, langsung aja ke{" "}
          <a href="https://github.com/teknologi-umum/hacktoberfest">
            teknologi-umum/hacktoberfest
          </a>{" "}
          buat kasih tau kita apa masalahnya.
          <p>
            Kalo kamu pengen langsung kontribusi buat bikin website ini jadi
            lebih bagus boleh banget!
          </p>
        </span>
      </footer>
    </>
  );
});
