import { useEffect, useState } from "preact/hooks";

export function AppTitle() {
  const [title, setTitle] = useState("");

  useEffect(() => {
    setTitle(document.title);
  }, []);

  return <h1>{title}</h1>;
}
