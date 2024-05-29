export type ErrorMessageProps = {
  message: string | undefined;
};

export const ErrorMessage = (props: ErrorMessageProps) => (
  <p className="text-red-500 mt-1 text-xs font-semibold">{props.message}</p>
);
