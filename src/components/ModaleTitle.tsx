export const ModaleTitle = ({
  src,
  blur = false,
}: {
  src: string | null;
  blur?: boolean;
}) => {
  return (
    <div className="pointer-events-none absolute inset-0">
      {src && (
        <img
          src={src}
          alt=""
          aria-hidden="true"
          className={`h-full w-full object-cover ${blur ? "scale-110 blur-2xl" : ""}`}
        />
      )}
      <div className="absolute inset-0 bg-gradient-to-r from-surface via-surface/85 to-surface/10" />
      <div className="absolute inset-0 bg-gradient-to-t from-surface via-transparent to-transparent" />
    </div>
  );
};
