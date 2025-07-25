import { html, render } from 'lit-html';
import { ic_carnage_backend } from 'declarations/ic-carnage-backend';
import logo from './logo2.svg';

class App {
  greeting = '';
  cars = [];

  constructor() {
    this.init();
  }

  async init() {
    await this.loadCars();
    this.#render();
  }

  async loadCars() {
    try {
      this.cars = await ic_carnage_backend.get_available_cars();
    } catch (error) {
      console.error('Failed to load cars: ', error);
    }
  }

  #handleSubmit = async (e) => {
    e.preventDefault();
    const name = document.getElementById('name').value;
    this.greeting = await ic_carnage_backend.greet(name);
    this.#render();
  };

  #render() {
    const carList = this.cars.map(
      (car) => html`
      <li>
        <strong>${car.name}</strong> (ID: ${car.id})<br/>
        Acceleration: ${car.acceleration}, Top Speed: ${car.top_speed}, Handling: ${car.handling}, Armor: ${car.armor}
      </li>
    `
    );
    let body = html`
      <main>
        <img src="${logo}" alt="DFINITY logo" />
        <br />
        <br />
        <h1>Welcome to IC Carnage!</h1>

        <form @submit=${this.handleSubmit}>
          <label for="name">Enter your name: </label>
          <input id="name" type="text" />
          <button type="submit">Click Me!</button>
        </form>

        <section id="greeting">${this.greeting}</section>

        <hr/>

        <h2>Available Cars</h2>
        <ul>${carList}</ul>
      </main>
    `;
    render(body, document.getElementById('root'));
    document
      .querySelector('form')
      .addEventListener('submit', this.#handleSubmit);
  }
}

export default App;
