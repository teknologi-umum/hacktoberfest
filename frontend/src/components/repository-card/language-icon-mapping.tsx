import {
  ClojureIcon,
  CSharpIcon,
  GoIcon,
  HandlebarsIcon,
  JavaIcon,
  JavascriptIcon,
  RubyIcon,
  RustIcon,
  TypescriptIcon,
} from "../icons";
import { type QwikJSX } from "@builder.io/qwik";

export const LANGUAGE_ICON_MAPPING: Record<string, QwikJSX.Element> = {
  javascript: <JavascriptIcon />,
  typescript: <TypescriptIcon />,
  java: <JavaIcon />,
  "c#": <CSharpIcon />,
  rust: <RustIcon />,
  go: <GoIcon />,
  handlebars: <HandlebarsIcon />,
  clojure: <ClojureIcon />,
  ruby: <RubyIcon />,
};
