import { useEffect, useState } from "react";
import { createActor } from "../../declarations/temp_backend";

const App = () => {
    const [tasks, setTasks] = useState([]);
    const [newTask, setNewTask] = useState("");
    const [editTaskId, setEditTaskId] = useState(null);
    const [editTaskTitle, setEditTaskTitle] = useState("");

    const backend = createActor(process.env.CANISTER_ID_TEMP_BACKEND);

    useEffect(() => {
        fetchTasks();
    }, []);

    const fetchTasks = async () => {
        try {
            const tasks = await backend.get_tasks();
            setTasks(tasks);
        } catch (error) {
            console.error("Error fetching tasks:", error);
        }
    };

    const addTask = async () => {
        if (!newTask) return;
        try {
            await backend.add_task(newTask);
            setNewTask("");
            fetchTasks();
        } catch (error) {
            console.error("Error adding task:", error);
        }
    };

    const deleteTask = async (id) => {
        try {
            await backend.delete_task(BigInt(id)); // ✅ Convert id to BigInt
            fetchTasks();
        } catch (error) {
            console.error("Error deleting task:", error);
        }
    };

    const updateTask = async (id) => {
        try {
            await backend.update_task(BigInt(id), [editTaskTitle], []);
            setEditTaskId(null);
            setEditTaskTitle("");
            fetchTasks();
        } catch (error) {
            console.error("Error updating task:", error);
        }
    };

    return (
        <div className="container">
            <h1>Web3 To-Do List</h1>

            {/* Input for Adding a Task */}
            <div>
                <input
                    type="text"
                    value={newTask}
                    onChange={(e) => setNewTask(e.target.value)}
                    placeholder="Enter a new task"
                />
                <button onClick={addTask}>Add Task</button>
            </div>

            {/* List of Tasks */}
            <ul>
                {tasks.map((task) => (
                    <li key={task.id}>
                        {editTaskId === task.id ? (
                            <>
                                <input
                                    type="text"
                                    value={editTaskTitle}
                                    onChange={(e) => setEditTaskTitle(e.target.value)}
                                />
                                <button onClick={() => updateTask(task.id)}>Save</button>
                                <button onClick={() => setEditTaskId(null)}>Cancel</button>
                            </>
                        ) : (
                            <>
                                <span>{task.title}</span>
                                <button onClick={() => deleteTask(task.id)}>Delete</button>
                                <button onClick={() => { setEditTaskId(task.id); setEditTaskTitle(task.title); }}>Edit</button>
                            </>
                        )}
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default App;
