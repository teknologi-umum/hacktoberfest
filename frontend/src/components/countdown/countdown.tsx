import {
  $,
  component$,
  useClientEffect$,
  useMount$,
  useStore,
  useStylesScoped$,
} from "@builder.io/qwik";
import { intervalToDuration } from "date-fns";
import styles from "./countdown.css?inline";

export const START_DATE = new Date("2023-10-01T00:00:00+07:00").getTime(); // 1st of October 2023 UTC+7
export const END_DATE = new Date("2023-10-31T00:00:00+07:00").getTime(); // 31st of October 2023 UTC+7
export const DAY = 24 * 60 * 60 * 1000; // a single day

type CountdownState = {
  days: string;
  hours: string;
  minutes: string;
  seconds: string;
  showCountdown: boolean;
};

export default component$(() => {
  useStylesScoped$(styles);

  const state = useStore<CountdownState>({
    days: "00",
    hours: "00",
    minutes: "00",
    seconds: "00",
    showCountdown: false,
  });

  const updateCountdown$ = $(() => {
    const now = Date.now();
    const { days, hours, minutes, seconds } = intervalToDuration({
      start: new Date(),
      end: now < START_DATE ? START_DATE : END_DATE + DAY, // it's inclusive so we need to add a day
    });

    // hide countdown when we're over with the event
    if (seconds === undefined) {
      state.showCountdown = false;
    }

    state.days = days?.toString().padStart(2, "0") ?? "00";
    state.hours = hours?.toString().padStart(2, "0") ?? "00";
    state.minutes = minutes?.toString().padStart(2, "0") ?? "00";
    state.seconds = seconds?.toString().padStart(2, "0") ?? "00";
    state.showCountdown = true;
  });

  // set initial date value from the server so we don't get 0 as our initial value
  useMount$(() => updateCountdown$());

  useClientEffect$(() => {
    const interval = setInterval(() => updateCountdown$(), 1000);

    return () => clearInterval(interval);
  });

  return (
    <div class="countdown" style={{ opacity: state.showCountdown ? "1" : "0" }}>
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
      <p class="countdown__title">
        {Date.now() > END_DATE ? "sejak" : "sampai"} Hacktoberfest{" "}
        {Date.now() < START_DATE ? "dimulai" : "berakhir"}
      </p>
    </div>
  );
});
