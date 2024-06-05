"use client";

import "chart.js/auto";

import { Chart, Interaction } from "chart.js";
import { CrosshairPlugin, Interpolate } from "chartjs-plugin-crosshair";
import { data2, data3, data4, data5 } from "../../utils/datas";
import { useEffect, useRef, useState } from "react";

import { Line } from "react-chartjs-2";
import { useCharts } from "../../hooks/useCharts";

Chart.register(CrosshairPlugin);
Interaction.modes.interpolate = Interpolate;

export default function Graph({ name, timestamp }) {
  const [data, setData] = useState({ loading: true, data: [], suggestedMax: 800000000 });
  const chartRef = useRef(null);
  const { copyGldGovSupply, copyGldGovTreasury } = useCharts();
  const [innerWidth, setInnerWidth] = useState(700);

  useEffect(() => setInnerWidth(window.innerWidth), []);

  useEffect(() => {
    (async () => {
      switch (name) {
        case "Treasury":
          if (copyGldGovTreasury?.loading) break;
          // await getTreasuryChart();
          // const data = await treasuryData({ timestamp });
          setData({
            loading: false,
            data: copyGldGovTreasury.data,
            suggestedMax: 800000000,
          });
          break;
        case "Staked":
          setData({ loading: false, data: data2, suggestedMax: 1200 });
          break;
        case "Liquid":
          setData({ loading: false, data: data3, suggestedMax: 1200 });
          break;
        case "Burned":
          setData({ loading: false, data: data4, suggestedMax: 1200 });
          break;
        case "Holders":
          setData({ loading: false, data: data5, suggestedMax: 1200 });
          break;
        case "Total GLDGov Supply":
          if (copyGldGovSupply?.loading) break;

          // await getSupplyChart();
          // await supplyData({ timestamp });
          setData({
            loading: false,
            data: copyGldGovSupply.data, //,
            suggestedMax: 1000010000,
            suggestedMin: 999905000,
          });
          break;
        default:
          //console.log("default");
          setData({ loading: false, data: [], suggestedMax: 1200 });
          break;
      }

      const chart = chartRef.current;
      const monthsCount = data.data?.length;
      if (chart && monthsCount > 0) {
        const xAxis = chart.scales.x;
        if (xAxis) {
          const stepSize = Math.ceil(monthsCount / 4);
          xAxis.options.gridLines = {
            display: true,
            drawBorder: false,
            drawOnChartArea: false,
            color: "#ccc",
            lineWidth: 1,
            tickLength: 0,
            borderDash: [5, 5],
            z: 0,

            drawTicks: function (context) {
              const ticks = xAxis.ticks;
              const tickStep = Math.floor(ticks.length / 4);
              for (let i = 0; i < ticks.length; i += tickStep) {
                const xPos = xAxis.getPixelForTick(i);
                context.save();
                context.beginPath();
                context.moveTo(xPos, 0);
                context.lineTo(xPos, chart.height);
                context.strokeStyle = this.color;
                context.lineWidth = this.lineWidth;
                context.setLineDash(this.borderDash);
                context.stroke();
                context.restore();
              }
            },
          };
          xAxis.options.ticks = {
            display: false,
          };
          chart.update();
        }
      }
    })();
  }, [
    copyGldGovSupply.data,
    copyGldGovSupply?.loading,
    copyGldGovTreasury.data,
    copyGldGovTreasury?.loading,
    data.data?.length,
    name,
  ]);

  return (
    <div className="mt-[30px]  h-[250px]">
      {data.loading && data.data.length > 0 ? (
        "loading"
      ) : (
        <Line
          ref={chartRef}
          data={{
            labels: data.data ? data.data.map((label) => label?.label) : [],
            datasets: [
              {
                data: data.data ? data.data.map((value) => value?.value) : [],
                label: name,
                backgroundColor: "rgba(211,184,113,0.2)",
                borderColor: "#D3B871",
                fill: true,
                pointStyle: false,
                backgroundColor: (context) => {
                  const bgColor = ["#D3B871CC", "#D3B87100"];
                  if (!context.chart.chartArea) return;
                  const {
                    ctx,
                    chartArea: { top, bottom },
                  } = context.chart;
                  const gradientBg = ctx.createLinearGradient(0, top, 0, bottom);
                  gradientBg.addColorStop(0, bgColor[0]);
                  gradientBg.addColorStop(1, bgColor[1]);
                  return gradientBg;
                },
              },
            ],
          }}
          options={{
            plugins: {
              tooltip: {
                mode: "nearest",
                intersect: false,
              },
              legend: {
                display: false,
              },
              label: {
                display: false,
              },
              crosshair: {
                line: {
                  color: "#D3B871CC",
                  widht: 1,
                },
              },
              sync: {
                enabled: true,
                group: 1,
                suppressTooltips: false,
              },
            },
            scales: {
              x: {
                display: true,
                type: "category",
                ticks: {
                  display: innerWidth < 641 ? false : true,
                },
                grid: {
                  display: true,
                },
              },
              y: {
                suggestedMin: data?.suggestedMin || 0,
                suggestedMax: data.suggestedMax,
                position: "right",
                ticks: {
                  callback: function (value, index, values) {
                    // Array of unit suffixes
                    var suffixes = ["", "K", "M", "B", "T"]; // Find the index of appropriate suffix
                    var suffixIndex = Math.floor(("" + value).length / 3); // Apply the suffix
                    if (suffixIndex >= 2) {
                      // If it's greater than or equal to "M", use "m" instead of "M"
                      suffixIndex = 2; // Set index to "M"
                      var shortValue =
                        parseFloat((value / Math.pow(1000, suffixIndex)).toFixed(1)) +
                        suffixes[suffixIndex].toLowerCase();
                    } else {
                      var shortValue =
                        parseFloat((value / Math.pow(1000, suffixIndex)).toFixed(1)) +
                        suffixes[suffixIndex];
                    }
                    return shortValue;
                  },
                  maxTicksLimit: 5, // Limit to 5 ticks
                  display: true,
                },
                grid: {
                  display: true,
                },
              },
            },
            responsive: true,
            maintainAspectRatio: false,
          }}
        />
      )}
    </div>
  );
}