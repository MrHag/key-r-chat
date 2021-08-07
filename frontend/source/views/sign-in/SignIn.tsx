import React from "react";
import { makeStyles } from "@material-ui/styles";
import { Modal, Button, Paper, Typography, TextField } from "@material-ui/core";
import { Redirect } from "react-router";

import { PropsFromConnector } from ".";
import { Form, Formik } from "formik";

const useStyles = makeStyles(() => ({
  paper: {
    position: "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    width: 400,
    borderRadius: "8px",
    outline: "none",
    padding: "16px",
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

interface IFormikValues {
  login: string;
  password: string;
}

const SignIn: React.FC<PropsFromConnector> = ({
  isAuthorized,
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

  return (
    <Modal
      open={true}
      aria-labelledby="simple-modal-title"
      aria-describedby="simple-modal-description"
    >
      <Paper className={classes.paper}>
        <Formik
          initialValues={{
            login: "",
            password: "",
          }}
          validate={formikValidation}
          onSubmit={(values) =>
            // values: IFormikValues
            // { setSubmitting }: FormikHelpers<IFormikValues>
            {
              actionSignIn(values.login, values.password);
            }
          }
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
              <Button variant="contained" color="secondary" type="submit">
                Login
              </Button>
            </Form>
          )}
        </Formik>
      </Paper>
    </Modal>
  );
};

export { SignIn };
