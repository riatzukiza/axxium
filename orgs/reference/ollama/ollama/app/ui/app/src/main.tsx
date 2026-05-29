import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { routeTree } from "./routeTree.gen";
import { fetchUser } from "./api";
import { StreamingProvider } from "./contexts/StreamingContext";

const queryClient = new QueryClient({
  defaultOptions: {
    mutations: {
      networkMode: "always", // Run mutations regardless of network state
    },
    queries: {
      networkMode: "always", // Allow queries even when offline (local server)
    },
  },
});

fetchUser().then((userData) => {
  if (userData) {
    queryClient.setQueryData(["user"], userData);
  }
});

const router = createRouter({
  routeTree,
  context: { queryClient },
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const REDACTED_SECRETElement = document.getElementById("REDACTED_SECRET")!;
if (!REDACTED_SECRETElement.innerHTML) {
  const REDACTED_SECRET = ReactDOM.createRoot(REDACTED_SECRETElement);
  REDACTED_SECRET.render(
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <StreamingProvider>
          <RouterProvider router={router} />
        </StreamingProvider>
      </QueryClientProvider>
    </StrictMode>,
  );
}
