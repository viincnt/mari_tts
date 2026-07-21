export type Lang = "en" | "pt";

export const LANGUAGES: { value: Lang; label: string }[] = [
  { value: "en", label: "English" },
  { value: "pt", label: "Português" },
];

export interface Sound {
  id: string;
  label: string;
  text: string;
  lang: Lang;
}
