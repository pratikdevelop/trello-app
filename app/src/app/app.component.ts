

import { Component, OnInit } from '@angular/core';
import { WeatherService, WeatherForecast } from './services/weather.service';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-root',
  imports:[CommonModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'Frontend';
  forecasts: WeatherForecast[] = [];

  constructor(private weatherService: WeatherService) {}

  ngOnInit() {
    this.weatherService.getWeatherForecast().subscribe(
      (data) => this.forecasts = data,
      (error) => console.error('Error fetching weather data', error)
    );
  }
}