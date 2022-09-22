import { component$, Slot } from "@builder.io/qwik";

export default component$(() => {
  return (
    <>
      <div class="starfield" />
      <div class="blue-nebula" />
      <div class="orange-nebula" />
      <main>
        <Slot />
      </main>
    </>
  );
});
