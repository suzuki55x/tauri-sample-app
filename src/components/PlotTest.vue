<template>
  <div id="app">
    <SigPlot id="plot1" :plot-options="{ xi: !btnToggle }">
      <ArrayLayer :plot-data="random" />
    </SigPlot>
    <SigPlot id="plot2">
      <HrefLayer :href="hrefData" />
    </SigPlot>
    <SigPlot id="plot3">
      <PipeLayer :pipe-data="random" :options="{type: 2000, subsize: 1000 }"/>
    </SigPlot>
    <SigPlot id="plot5" :plot-options="plotOptions">
      <ArrayLayer :plot-data="plotdata" :options="dataHeader" :layerOptions="layerOptions"/>
    </SigPlot>
    <SigPlot id="plot4" :plot-options="plotOptions2">
      <ArrayLayer :plot-data="random2" :options="dataHeader2" :layerOptions="layerOptions2"/>
    </SigPlot>
    <button id="toggler" @click="btnToggle = !btnToggle">Toggle Data</button>
    <button id="toggler" @click="read_file">READ</button>
    {{plotdata}}
  </div>
</template>

<script>
import SigPlot from "./SigPlot.vue";
import ArrayLayer from "./ArrayLayer.vue";
import HrefLayer from "./HrefLayer.vue";
import PipeLayer from "./PipeLayer.vue";

import { invoke } from "@tauri-apps/api/tauri";
import { dialog} from "@tauri-apps/api";

export default {
  name: "App",
  components: {
    SigPlot,
    ArrayLayer,
    HrefLayer,
    PipeLayer
  },
  computed: {
    hrefData() {
      return this.btnToggle ? this.href1 : this.href2
    },
  },
  data() {
    return {
      btnToggle: false,
      href1: "https://sigplot.lgsinnovations.com/dat/penny.prm",
      href2: "https://sigplot.lgsinnovations.com/dat/sin.tmp",
      random: [],
      random2: [],
      random2D: [],
      plotdata: [0,0],
      test: [1,2,2,1,4,4,2,3,1,1, 0.1, 2],
      generateDataInterval: 0,
      plotOptions: {
        cmode: 5,
        ymin: -1000,
        ymax: 1000,
        xmin: -1000,
        xmax: 1000
      },
      plotOptions2: {
        cmode: 5,
        ymin: -2,
        ymax: 2,
        xmin: -2,
        xmax: 2
      },
      dataHeader: {
        xunits: "Q",
        yunits: "I"
      },
      dataHeader2: {
        xunits: "Q",
        yunits: "I"
      },
      layerOptions: {
        name: "IQ data",
        mode: "XY",
        framesize: 512,
        line: 0,
        radius: 1,
        symbol: 3
      },
      layerOptions2: {
        name: "IQ data2",
        mode: "XY",
        framesize: 512,
        line: 0,
        radius: 1,
        symbol: 3
      },
    }
  },
  beforeDestroy() {
    clearInterval(this.generateDataInterval)
  },
  mounted() {
    this.generateData()
  },
  methods: {
    generateData() {
      this.generateDataInterval = setInterval(() => {
        let random = [];
        let random2 = [];
        let random2D = [];
        for (let i = 0; i < 1000; i += 1) {
          random.push(Math.random());
          random2.push(Math.random()*2-1);
          let tmp = [];
          for (let j = 0; j < 2; j += 1) {
            tmp.push(Math.random()*2-1);
          }
          random2D.push(tmp);
        }
        this.random = random;
        this.random2D = random2D;
        this.random2 = random2;
      }, 16);
    },
    async read_file() {
      let filepath = await dialog.open()
      this.plotdata = await invoke("read_file_command", {path: filepath})
    }
  }
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
#plot1,
#plot2,
#plot4,
#plot5,
#plot3 {
  display: inline-block;
  height: 400px;
  width: 400px;
    margin: 10px;
}
  #toggler {
    height: 30px;
    width: 100px;
      display: block;
    background: none;
    border:1px solid gray;
    border-radius: 3px;
  }
  #toggler:active {
    box-shadow: inset 0 2px 3px 0 black;
  }
  #toggler:focus {
    outline: none
  }
</style>