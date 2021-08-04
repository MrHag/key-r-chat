import { compose, createStore, applyMiddleware } from "redux";
import { rootReducer, IAppStore } from "./root-reducer";
import thunk from "redux-thunk";

declare global {
  interface Window {
    __REDUX_DEVTOOLS_EXTENSION_COMPOSE__?: typeof compose;
  }
}

// const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__;

// const appStore = createStore(
//   rootReducer,
//   {},
//   compose(applyMiddleware(thunk), composeEnhancers)
// );

const initialState = {};
const enhancers = [];
const middleware = [thunk];

if (process.env.NODE_ENV === "development") {
  const devToolsExtension =
    ((window as any).__REDUX_DEVTOOLS_EXTENSION__ &&
      (window as any).__REDUX_DEVTOOLS_EXTENSION__()) ||
    compose;
  if (typeof devToolsExtension === "function") {
    enhancers.push(devToolsExtension);
  }
}

const composedEnhancers = compose(applyMiddleware(...middleware), ...enhancers);

const appStore = createStore(rootReducer, initialState, composedEnhancers);

export { IAppStore, appStore };
