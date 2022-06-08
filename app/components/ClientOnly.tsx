import { FC, ReactElement, useEffect, useState } from "react";

const ClientOnly: FC<{ children: ReactElement }> = ({ children }) => {
  const [hasMounted, setHasMounted] = useState(false);

  useEffect(() => {
    setHasMounted(true);
  }, []);

  return hasMounted ? children : null;
};

export { ClientOnly };
