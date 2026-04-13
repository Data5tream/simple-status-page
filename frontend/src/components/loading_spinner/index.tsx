import "./styles.css";

interface LoadingSpinnerProps {
  color: string;
}

export function LoadingSpinner({ color }: LoadingSpinnerProps) {
  return <div class="loading_spinner" style={color} />;
}
