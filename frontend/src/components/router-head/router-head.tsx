import { component$ } from "@builder.io/qwik";
import { useDocumentHead, useLocation } from "@builder.io/qwik-city";

/**
 * The RouterHead component is placed inside of the document `<head>` element.
 */
export const RouterHead = component$(() => {
  const head = useDocumentHead();
  const loc = useLocation();

  return (
    <>
      <title>{head.title}</title>

      <link rel="canonical" href={loc.href} />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <link rel="shortcut icon" type="image/png" href="/favicon.png" />
      <link rel="shortcut icon" href="/favicon.ico" />
      <link rel="icon" type="image/svg+xml" href="/favicon.svg" />

      <meta property="og:site_name" content="Teknologi Umum Hacktoberfest" />
      <meta name="twitter:site" content="hacktoberfest.teknologiumum.com" />
      <meta name="twitter:title" content="Teknologi Umum Hacktoberfest" />

      <meta
        name="keywords"
        content="hacktoberfest, teknologi umum, programming, developer"
      />
      <meta
        name="description"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />
      <meta
        name="subject"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />
      <meta name="copyright" content="Teknologi Umum" />
      <meta name="language" content="en-US" />
      <meta name="robots" content="index,follow" />
      <meta
        name="summary"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />
      <meta name="author" content="Teknologi Umum, teknologi.umum@gmail.com" />
      <meta name="owner" content="Teknologi Umum" />

      <meta name="url" content="https://hacktoberfest.teknologiumum.com/" />
      <meta
        name="identifier-URL"
        content="https://hacktoberfest.teknologiumum.com/"
      />
      <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
      <link rel="canonical" href="https://hacktoberfest.teknologiumum.com/" />

      <meta property="og:title" content="Hacktoberfest Teknologi Umum" />
      <meta property="og:type" content="website" />
      <meta
        property="og:url"
        content="https://hacktoberfest.teknologiumum.com/"
      />
      <meta
        property="og:image"
        content="https://hacktoberfest.teknologiumum.com/social.jpg"
      />
      <meta
        property="og:image:secure:url"
        content="https://hacktoberfest.teknologiumum.com/social.jpg"
      />
      <meta
        property="og:image:alt"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />
      <meta property="og:site_name" content="Hacktoberfest Teknologi Umum" />
      <meta property="og:locale" content="id-ID" />
      <meta
        property="og:description"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />

      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:site" content="" />
      <meta name="twitter:creator" content="Teknologi Umum" />
      <meta name="twitter:title" content="Hacktoberfest Teknologi Umum" />
      <meta
        name="twitter:description"
        content="Website yang memuat semua issue untuk kebutuhan Hacktoberfest dari organisasi Teknologi Umum"
      />
      <meta
        name="twitter:image"
        content="https://hacktoberfest.teknologiumum.com/social.jpg"
      />

      {head.meta.map((m) => (
        <meta {...m} />
      ))}

      {head.links.map((l) => (
        <link {...l} />
      ))}

      {head.styles.map((s) => (
        <style {...s.props} dangerouslySetInnerHTML={s.style} />
      ))}
    </>
  );
});
