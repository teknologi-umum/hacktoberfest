import { component$, Slot } from "@builder.io/qwik";

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
          Have any suggestions for this website? Visit{" "}
          <a href="https://github.com/teknologi-umum/hacktoberfest">
            teknologi-umum/hacktoberfest
          </a>{" "}
          and let us know what you think! ツ
        </span>
      </footer>
    </>
  );
});
