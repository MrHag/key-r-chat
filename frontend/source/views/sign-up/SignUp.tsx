import React from "react";
import { makeStyles } from "@material-ui/styles";
import { Button, Typography, TextField, Link } from "@material-ui/core";
import { Redirect, Link as RouterLink } from "react-router-dom";
import { View } from "views/view";
import { SIGN_IN } from "constants/routes";
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
  passwordField: {
    marginBottom: "8px",
  },
  form: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
}));

interface IFormikValues {
  login: string;
  password: string;
  confirmPassword: string;
}

const SignUp: React.FC<PropsFromConnector> = ({
  isAuthorized,
  actionSignIn,
}: PropsFromConnector) => {
  const classes = useStyles();

  const formikValidation = (values: IFormikValues) => {
    const errors: {
      login?: string;
      password?: string;
      confirmPassword?: string;
    } = {};
    if (values.login === "") {
      errors.login = "Login must not be empty!";
    }
    if (values.password === "") {
      errors.password = "Password must not be empty!";
    }
    if (values.confirmPassword === "") {
      errors.confirmPassword = "Enter confirm password!";
    }
    if (values.password != values.confirmPassword) {
      errors.confirmPassword = "Password don't match!";
    }
    return errors;
  };

  if (isAuthorized) {
    return <Redirect to="/" />;
  }

  return (
    <View className={classes.root}>
      <Formik
        initialValues={{
          login: "",
          password: "",
          confirmPassword: "",
        }}
        validate={formikValidation}
        onSubmit={(values) => {
          actionSignIn(values.login, values.password);
        }}
      >
        {(formik) => (
          <Form className={classes.form}>
            <Typography variant="h6" className={classes.title}>
              Sign up
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
              className={classes.passwordField}
              autoComplete="off"
              id="password"
              name="password"
              label="Password"
              value={formik.values.password}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.password)}
              helperText={formik.errors.password}
            />
            <TextField
              className={classes.textField}
              autoComplete="off"
              id="confirmPassword"
              name="confirmPassword"
              label="Confirm password"
              value={formik.values.confirmPassword}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.confirmPassword)}
              helperText={formik.errors.confirmPassword}
            />
            <Link
              component={RouterLink}
              to={SIGN_IN}
              className={classes.textField}
            >
              Already have an account?
            </Link>
            <Button variant="contained" color="primary" type="submit">
              Sign up
            </Button>
          </Form>
        )}
      </Formik>
    </View>
  );
};

export { SignUp };
