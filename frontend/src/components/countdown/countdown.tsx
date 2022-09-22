import {
  $,
  component$,
  useClientEffect$,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import styles from "./countdown.css";
import { intervalToDuration } from "date-fns";

export const START_DATE = new Date("2022-10-01T00:00:00+07:00").getTime(); // 1st of October 2022 UTC+7
export const END_DATE = new Date("2022-10-31T00:00:00+07:00").getTime(); // 31st of October 2022 UTC+7

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore({
    days: " ",
    hours: " ",
    minutes: " ",
    seconds: " ",
  });

  useClientEffect$(() => {
    const interval = setInterval(() => {
      const now = Date.now();
      const { days, hours, minutes, seconds } = intervalToDuration({
        start: new Date(),
        end: now < START_DATE ? START_DATE : END_DATE,
      });
      console.log({ days, hours, minutes, seconds });
      state.days = days?.toString().padStart(2, "0") ?? " ";
      state.hours = hours?.toString().padStart(2, "0") ?? " ";
      state.minutes = minutes?.toString().padStart(2, "0") ?? " ";
      state.seconds = seconds?.toString().padStart(2, "0") ?? " ";
    }, 1000);

    return () => clearInterval(interval);
  });

  return (
    <div class="countdown">
      <div class="countdown__items">
        <div class="countdown__item">
          <span>{state.days}</span> hari
        </div>
        <div class="countdown__item">
          <span>{state.hours}</span> jam
        </div>
        <div class="countdown__item">
          <span>{state.minutes}</span> menit
        </div>
        <div class="countdown__item">
          <span>{state.seconds}</span> detik
        </div>
      </div>
      <p class="countdown__title">sampai Hacktoberfest dimulai</p>
    </div>
  );
});
