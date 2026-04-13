import { LoadingSpinner } from "../loading_spinner/index.tsx";

import "./styles.css";

export function LoadingOverlay() {
  return (
    <div className="loading">
      <LoadingSpinner color="var(--c-white)" />
      <span>Loading data...</span>
    </div>
  );
}
