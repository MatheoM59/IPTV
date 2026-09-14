import { X } from "lucide-react";

export const Modale = ({
  children,
  onClose,
}: {
  children: React.ReactNode;
  onClose: () => void;
}) => {
  return (
    <div className="fixed inset-35 z-5 flex flex-col overflow-hidden rounded-panel border border-line bg-surface">
      <button
        type="button"
        onClick={onClose}
        aria-label="Fermer"
        className="absolute right-3 top-3 z-20 grid size-8 place-items-center rounded-full
          border border-line-strong bg-black/60 text-ink transition-colors hover:bg-black/80
          focus-visible:outline-2 focus-visible:outline-accent"
      >
        <X className="size-4" />
      </button>
      {children}
    </div>
  );
};
