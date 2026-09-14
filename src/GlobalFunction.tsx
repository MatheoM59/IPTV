export const formatExpDate = (string: string | null | undefined) => {
  if (!string) {
    return;
  }
  const stringToExpdate = Number(string) * 1000;
  const date = new Date(stringToExpdate);
  const expDate = date.toLocaleDateString("fr-FR", {
    day: "numeric",
    month: "long",
    year: "numeric",
  });
  return expDate;
};

export const formatSearch = (string: string) => {
  const stringLow = string
    .toLowerCase()
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "");

  return stringLow;
};
