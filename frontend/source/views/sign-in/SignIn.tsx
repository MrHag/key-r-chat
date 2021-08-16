import React from "react";
import { makeStyles, withStyles } from "@material-ui/styles";
import { Button, Typography, TextField, Link } from "@material-ui/core";
import { Redirect, Link as RouterLink } from "react-router-dom";
import { View } from "views/view";
import { SIGN_UP } from "constants/routes";
import { PropsFromConnector } from ".";
import { Form, Formik } from "formik";

const useStyles = makeStyles(() => ({
  root: {
    justifyContent: "center",
  },
  title: {
    marginBottom: "32px",
  },
  textField: {
    marginBottom: "32px",
  },
  form: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
}));

const ErrorMsg = withStyles({
  root: {
    color: "red",
    marginBottom: "32px",
    fontSize: "14px",
  },
})(Typography);

interface IFormikValues {
  login: string;
  password: string;
}

const SignIn: React.FC<PropsFromConnector> = ({
  isAuthorized,
  signIn,
  actionSignIn,
}: PropsFromConnector) => {
  const classes = useStyles();

  const formikValidation = (values: IFormikValues) => {
    const errors: { login?: string; password?: string } = {};
    if (values.login === "") {
      errors.login = "Login must not be empty!";
    }
    if (values.password === "") {
      errors.password = "Password must not be empty!";
    }
    return errors;
  };

  if (isAuthorized) {
    return <Redirect to="/" />;
  }

  const errorMsg = signIn?.errorMsg && (
    <ErrorMsg variant="h6">{signIn.errorMsg}</ErrorMsg>
  );

  return (
    <View className={classes.root}>
      <Formik
        initialValues={{
          login: "",
          password: "",
        }}
        validate={formikValidation}
        onSubmit={(values) => {
          actionSignIn(values.login, values.password);
        }}
      >
        {(formik) => (
          <Form className={classes.form}>
            <Typography variant="h6" className={classes.title}>
              Sign in
            </Typography>
            <TextField
              autoFocus={true}
              className={classes.textField}
              autoComplete="off"
              id="login"
              name="login"
              label="Login"
              value={formik.values.login}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.login)}
              helperText={formik.errors.login}
            />
            <TextField
              className={classes.textField}
              autoComplete="off"
              id="password"
              name="password"
              label="Password"
              value={formik.values.password}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.password)}
              helperText={formik.errors.password}
            />
            {errorMsg}
            <Link
              component={RouterLink}
              to={SIGN_UP}
              className={classes.textField}
            >
              {"Don't have an acoount?"}
            </Link>
            <Button variant="contained" color="primary" type="submit">
              Login
            </Button>
          </Form>
        )}
      </Formik>
    </View>
  );
};

export { SignIn };
