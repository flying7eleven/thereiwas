import { FormEvent, useRef, useState } from "react";
import Avatar from "@mui/material/Avatar";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Paper from "@mui/material/Paper";
import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
import LockOutlinedIcon from "@mui/icons-material/LockOutlined";
import Typography from "@mui/material/Typography";
import { useLocation, useNavigate } from "react-router-dom";
import { useAuthentication } from "../../hooks/useAuthentication";
import { Alert, Snackbar } from "@mui/material";

interface Props {
  isDark: boolean;
}

export const LoginView = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();
  const auth = useAuthentication();

  const state = location.state as { from: Location };
  const from = state ? state.from.pathname : "/";

  const usernameField = useRef<HTMLInputElement>(null);
  const passwordField = useRef<HTMLInputElement>(null);

  const [isSnackbarOpen, setIsSnackbarOpen] = useState<boolean>(false);

  const handleSnackbarClose = () => {
    setIsSnackbarOpen(false);
  };

  const requestAuthorizationToken = (event: FormEvent) => {
    // ensure that we do not handle the actual submit event anymore
    event.preventDefault();

    // ensure the error snack bar is not visible before sending the request
    setIsSnackbarOpen(false);

    // try to authenticate against the API backend
    auth.signin(
      usernameField?.current?.value || "",
      passwordField?.current?.value || "",
      () => {
        // Send them back to the page they tried to visit when they were
        // redirected to the login page. Use { replace: true } so we don't create
        // another entry in the history stack for the login page.  This means that
        // when they get to the protected page and click the back button, they
        // won't end up back on the login page, which is also really nice for the
        // user experience.
        navigate(from, { replace: true });
      },
      () => setIsSnackbarOpen(true),
    );
  };

  const getCorrectImage = () => {
    if (props.isDark) {
      return "public/background_login_dark.jpg";
    }
    return "public/background_login_light.jpg";
  };

  return (
    <>
      <Snackbar
        open={isSnackbarOpen}
        autoHideDuration={6000}
        onClose={handleSnackbarClose}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        <Alert
          onClose={handleSnackbarClose}
          severity="error"
          sx={{ width: "100%" }}
        >
          login.alerts.failed_login.message
        </Alert>
      </Snackbar>
      <Grid container component="main" sx={{ height: "100vh" }}>
        <Grid
          item
          xs={false}
          sm={4}
          md={7}
          sx={{
            backgroundImage: `url(${getCorrectImage()})`,
            backgroundRepeat: "no-repeat",
            backgroundColor: (t) =>
              t.palette.mode === "light"
                ? t.palette.grey[50]
                : t.palette.grey[900],
            backgroundSize: "cover",
            backgroundPosition: "center",
          }}
        />
        <Grid item xs={12} sm={8} md={5} component={Paper} elevation={6} square>
          <Box
            sx={{
              my: 8,
              mx: 4,
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
            }}
          >
            <Avatar sx={{ m: 1, bgcolor: "secondary.main" }}>
              <LockOutlinedIcon />
            </Avatar>
            <Typography component="h1" variant="h5">
              login.headlines.signin
            </Typography>
            <Box
              component="form"
              noValidate
              onSubmit={requestAuthorizationToken}
              sx={{ mt: 1 }}
            >
              <TextField
                margin="normal"
                required
                fullWidth
                id="username"
                label="login.form.username_field_label"
                name="username"
                autoComplete="username"
                autoFocus
                inputRef={usernameField}
              />
              <TextField
                margin="normal"
                required
                fullWidth
                name="password"
                label="login.form.password_field_label"
                type="password"
                id="password"
                autoComplete="current-password"
                inputRef={passwordField}
              />
              <Button
                type="submit"
                fullWidth
                variant="contained"
                sx={{ mt: 3, mb: 2 }}
              >
                login.form.sign_in_button
              </Button>
            </Box>
          </Box>
        </Grid>
      </Grid>
    </>
  );
};
