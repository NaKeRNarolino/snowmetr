import { useEffect, useState } from "react";

interface AsyncState<T> {
  data: T | null;
  error: Error | null;
  loading: boolean;
}

// The type for the async function
type AsyncFunction<T> = () => Promise<T>;

// The custom useAsync hook
export function useAsync<T>(
  asyncFunction: AsyncFunction<T>,
  dependencies: any[] = []
): AsyncState<T> {
  const [state, setState] = useState<AsyncState<T>>({
    data: null,
    error: null,
    loading: true,
  });

  useEffect(() => {
    let isMounted = true; // Track whether the component is still mounted

    setState({ data: null, error: null, loading: true });

    asyncFunction()
      .then((data) => {
        if (isMounted) {
          setState({ data, error: null, loading: false });
        }
      })
      .catch((error) => {
        if (isMounted) {
          setState({ data: null, error, loading: false });
        }
      });

    return () => {
      isMounted = false; // Cleanup on unmount
    };
  }, dependencies);

  return state;
}
