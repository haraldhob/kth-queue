import { applyMiddleware, createStore, combineReducers } from 'redux';
import thunk from 'redux-thunk';
import promise from 'redux-promise-middleware';
import { composeWithDevTools } from 'redux-devtools-extension/logOnlyInProduction';
import userReducer from './reducers/userReducer';
import queueReducer from './reducers/queueReducer';
import socketReducer from './reducers/socketReducer';
import administratorReducer from './reducers/administratorReducer';
import modalReducer from './reducers/modalReducer';
import User from './models/User';
import Queue from './models/Queue';
import Administrator from './models/Administrator';
import Modal from './models/Modal';

export interface GlobalStore {
  user: User | null,
  queues: Queue[],
  administration: {
    administrators: Administrator[]
    selectedQueue: string
  },
  modals: Modal[]
}

const middleware = applyMiddleware(promise, thunk);

const reducer = combineReducers({
  user: userReducer,
  queues: queueReducer,
  socket: socketReducer,
  administration: administratorReducer,
  modals: modalReducer
});

const store = createStore(
  reducer,
  composeWithDevTools(middleware));

export default store;
