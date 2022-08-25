import 'regenerator-runtime/runtime';
import React from 'react';

import {addTaskToList,completeTask,getToDoList} from './near-api';
import {SignInPrompt, SignOutButton } from './ui-components';


export default function App() {
  const [listFromBlockchain, setListFromBlockchain] = React.useState([]);
  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  // Get blockchian state once on component load
  React.useEffect(() => {
    getToDoList()
      .then(setListFromBlockchain)
      .then(data => console.log(data))
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }, []);

  /// If user not signed-in with wallet - show prompt
  if (!window.walletConnection.isSignedIn()) {
    // Sign-in flow will reload the page later
    return <SignInPrompt/>;
  }

  function addNewTask(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { taskInput } = document.getElementById("newTask").value;
    addTaskToList(taskInput)
      .then(getToDoList)
      .catch(setListFromBlockchain)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }

  function onComplete(e){
    e.preventDefault();
    setUiPleaseWait(true);
    const {taskComplete}= e.target.elements.id;
    completeTask(parseInt(taskComplete))
      .then(()=> document.getElementById(taskComplete).disabled = true)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }
  return(
    <>
      <SignOutButton accountId={window.accountId}/>
      <main className={uiPleaseWait ? 'please-wait' : ''}>
        {Array.isArray(listFromBlockchain)?listFromBlockchain.forEach((job,index)=>(
          <>
          <input type="text" id={"task "+job.id} placeholder={job.task} disabled/><button id={job.id} onClick={onComplete}>Complete</button><br/>
          </>
        )): null}
        <input type="text" autoComplete="off" id='newTask'/>
        <button onClick={addNewTask}>Add new task</button>
      </main>
    </>
  )
}
