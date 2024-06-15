// referenced https://github.com/bezkoder/react-typescript-login-example to make this class
const eventBus = {
    on(event: string, callback: EventListener) {
      document.addEventListener(event, (e) => callback(e));
    },
    dispatch(event: string, data?: any) {
      document.dispatchEvent(new CustomEvent(event, { detail: data }));
    },
    remove(event: string, callback: EventListener) {
      document.removeEventListener(event, callback);
    },
  };
  
  export default eventBus;