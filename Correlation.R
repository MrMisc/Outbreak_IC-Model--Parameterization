#! /usr/bin/Rscript

setwd("E://Outbreak_Parameterization")
data<-read.csv("output.csv",header = FALSE)
data <- data %>%
  mutate(Floored_V8 = floor(V8))
data$V8<-data$Floored_V8
library(ggplot2)
library(plotly)
library(thematic)
library(extrafont)
library(pandoc)
library(plotly)
library(dplyr)
fig_dots<-subset(data,V9<quantile(V9,0.99))%>%plot_ly(type="scatter",
          mode = "markers")%>%
  add_trace(x = ~V8,
          y = ~V6,
          color ="MSE Points",
          colors=c("#2A6074","#00C9B1"),
          size = ~1/V9,
          text = ~paste("Factor 1: ", V8, "<br>Factor 2: ", V6, "<br>MSE Discrepancy: ", V9))%>%
  layout(title = "Correlation/Percentile plot",
         plot_bgcolor = '#FFF8EE',
         xaxis = list(
          title = "Factor 8",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'),
         yaxis = list(
          title = "Factor 5",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'))
htmlwidgets::saveWidget(fig_dots, "Correlation_99th.html", selfcontained = TRUE)

fig_dots<-subset(data,V9<quantile(V9,0.1))%>%plot_ly(type="scatter",
                                                      mode = "markers")%>%
  add_trace(x = ~V8,
            y = ~V6,
            color ="MSE Points",
            colors=c("#2A6074","#00C9B1"),
            size = ~1/V9,
            text = ~paste("Factor 1: ", V8, "<br>Factor 2: ", V6, "<br>MSE Discrepancy: ", V9))%>%
  layout(title = "Correlation/Percentile plot",
         plot_bgcolor = '#FFF8EE',
         xaxis = list(
           title = "Factor 8",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'),
         yaxis = list(
           title = "Factor 5",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'))
htmlwidgets::saveWidget(fig_dots, "Correlation_10th.html", selfcontained = TRUE)



##Better plots compared to Rust
##Factor 5/6 as eg -> PD
fig_dots<-subset(data,V9<quantile(V9,0.1))%>%plot_ly(type="scatter",
                                                      mode = "markers")%>%
  add_trace(x = ~V6,
            y = ~V9,
            color ="MSE Points",
            colors=c("#2A6074","#00C9B1"),
            size = ~V8,
            text = ~paste("Factor 1: ", V8, "<br>Factor 2: ", V6, "<br>MSE Discrepancy: ", V9))%>%
  layout(title = "Correlation/Percentile plot",
         plot_bgcolor = '#FFF8EE',
         xaxis = list(
           title = "Factor 5",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'),
         yaxis = list(
           title = "MSE Discrepancy",
           zerolinecolor = '#ffff',
           zerolinewidth = 0.5,
           gridcolor = '#F4F2F0'))
fig_dots
