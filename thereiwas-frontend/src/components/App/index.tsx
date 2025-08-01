import { useMemo } from "react";
import useMediaQuery from "@mui/material/useMediaQuery";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import CssBaseline from "@mui/material/CssBaseline";
import { DashboardView } from "../../views/DashboardView";
import { AuthenticationProvider } from "../../provider/AuthenticationProvider";
import { RequireAuthentication } from "../RequireAuthentication";
import { AuthenticatedView } from "../../views/AuthenticatedView";
import { LoginView } from "../../views/LoginView";

export const App = () => {
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode],
  );

  return (
    <BrowserRouter>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <AuthenticationProvider>
          <Routes>
            <Route>
              <Route
                path="/"
                element={
                  <RequireAuthentication>
                    <AuthenticatedView content={<DashboardView />} />
                  </RequireAuthentication>
                }
              />
              <Route
                path="/login"
                element={<LoginView isDark={prefersDarkMode} />}
              />
            </Route>
          </Routes>
        </AuthenticationProvider>
      </ThemeProvider>
    </BrowserRouter>
  );
};
