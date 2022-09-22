import { component$, Slot } from "@builder.io/qwik";

export default component$(() => {
  return (
    <>
      <div class="starfield" />
      <div class="blue-nebula" />
      <div class="orange-nebula" />
      <div class="pink-nebula" />
      <main>
        <Slot />
      </main>
    </>
  );
});
