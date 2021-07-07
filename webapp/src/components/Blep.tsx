import { useContext } from "solid-js";
import { CounterContext } from "../App";

export default function () {
	const [state] = useContext(CounterContext)!;
	console.log(state.count);
	return <div>Blep</div>;
}
